#!/usr/bin/env perl

use v5.32;
use strict;
use warnings;
use feature 'signatures';

package Creator;

use autodie qw( :all );

use IPC::Run3 qw( run3 );
use LWP::Simple qw( get getstore RC_OK );
use Path::Tiny qw( tempdir );
use Specio::Library::String;
use Specio::Library::Path::Tiny;

use Moose;

with 'MooseX::Getopt::Dashes';

no warnings 'experimental::signatures';

MooseX::Getopt::OptionTypeMap->add_option_type_to_map(
    t('Dir') => '=s',
);

has data_dir => (
    is       => 'ro',
    isa      => t('Dir'),
    coerce   => 1,
    required => 1,
);

has token => (
    is       => 'ro',
    isa      => t('NonEmptyStr'),
    required => 1,
);

has db_owner => (
    is      => 'ro',
    isa     => t('NonEmptyStr'),
    default => sub {getlogin},
);

has _temp_dir => (
    is      => 'ro',
    isa     => t('Dir'),
    lazy    => 1,
    default => sub { return tempdir() },
);

my @DumpFiles = qw( mbdump.tar.bz2 mbdump-cover-art-archive.tar.bz2 );

sub run ($self) {
    local $ENV{MBSLAVE_CONFIG} = $self->_write_mbslave_config;

    $self->_create_database;
    $self->_download_data;
    $self->_create_schema;
    $self->_import_data;
    $self->_run_post_import_sql;
    $self->_vacuum;

    return 0;
}

sub _create_database ($self) {
    $self->_create_user;
    _run(
        qw( psql --quiet template0 --command ),
        'CREATE DATABASE IF NOT EXISTS music_player',
    );
    _run(
        qw( psql --quiet music_player --command ),
        'CREATE EXTENSION cube',
    );
    _run(
        qw( psql --quiet music_player --command ),
        'CREATE EXTENSION earthdistance',
    );
}

sub _download_data ($self) {
    my $base_url
        = 'http://ftp.musicbrainz.org/pub/musicbrainz/data/fullexport';
    my $listing = get($base_url)
        or die "Could not GET $base_url";

    my @dump_dirs = $listing =~ m{<a href="(\d{8}-\d{6})/"}g
        or die
        "Could not find any data dump dirs in the listing HTML:\n$listing\n";
    my $latest = ( sort @dump_dirs )[-1];

    for my $file (@DumpFiles) {
        my $target = $self->data_dir->child($file);
        if ( $target->exists ) {
            say "Skipping $target because we already have it"
                or die $!;
            next;
        }

        my $url = "$base_url/$latest/$file";
        say "Downloading $url"
            or die $!;
        my $code = getstore( $url, "$target" );
        die "Could not download $url"
            unless $code == RC_OK;
    }
}

my $config = <<'EOF';
[database]
host = 127.0.0.1
port = 5432
name = music_player
user = {USER}

[musicbrainz]
base_url = https://metabrainz.org/api/musicbrainz/
token = {TOKEN}

[schemas]
musicbrainz = musicbrainz
cover_art_archive = musicbrainz
ignore = documentation,event_art_archive,statistics,wikidocs
EOF

sub _write_mbslave_config ($self) {
    $config =~ s/\{USER\}/$self->db_owner/e;
    $config =~ s/\{TOKEN\}/$self->token/e;

    my $file = $self->_temp_dir->child('mbslave.conf');
    $file->spew($config);
    return $file;
}

sub _create_schema ($self) {
    say 'Creating schema'
        or die $!;
    run3(
        [qw( mbslave psql -S )],
        \'CREATE SCHEMA musicbrainz',
        undef,
        undef,
    );

    _run(qw( mbslave psql -f CreateTables.sql ));
    _run(qw( mbslave psql -f caa/CreateTables.sql ));
}

sub _import_data ($self) {
    for my $file (@DumpFiles) {
        _run( qw( mbslave import ), $self->data_dir->child($file) );
    }
}

sub _run_post_import_sql {

    # XXX - we need to fix CreateFunctions to s/anyarray/anycompatiblearray/
    # and s/anyelement/anycompatible/. Not sure how best to do this. For now
    # I've manually edited my local version.
    my @sql = qw(
        CreateFunctions.sql
        CreateIndexes.sql
        CreatePrimaryKeys.sql
        CreateSlaveIndexes.sql
        CreateViews.sql
        caa/CreateIndexes.sql
        caa/CreatePrimaryKeys.sql
    );

    for my $sql (@sql) {
        _run( qw( mbslave psql -s musicbrainz -f ), $sql );
    }
}

sub _vacuum {
    say 'Vacuuming'
        or die $!;
    run3(
        [qw( mbslave psql -S )],
        \'VACUUM ANALYZE',
        undef,
        undef,
    );
}

sub _run (@cmd) {
    say "Running: @cmd"
        or die $!;
    system(@cmd);
}

package main;

exit Creator->new_with_options->run;
