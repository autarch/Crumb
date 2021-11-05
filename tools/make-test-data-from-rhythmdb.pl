#!/usr/bin/env perl

use v5.32;
use strict;
use warnings;
use feature 'signatures';

package Maker;

use autodie qw( :all );

use JSON::MaybeXS qw( JSON );
use List::AllUtils qw( all );
use MP3::Tag;
use Path::Tiny qw( path );
use Path::Tiny::Rule;

use Moose;

with 'MooseX::Getopt::Dashes';

no warnings 'experimental::signatures';

sub run ($self) {
    my ( $artists, $albums ) = $self->_scan_mp3s;

    path('artists.json')->spew_utf8( JSON()->new->pretty->encode($artists) );
    path('albums.json')->spew_utf8( JSON()->new->pretty->encode($albums) );

    return 0;
}

sub _scan_mp3s ($self) {
    my ( %artists, %albums );

    my $iter
        = Path::Tiny::Rule->new->file->name(qr/\.mp3$/)
        ->iter(
        '/home/autarch/mnt/music');
    my $x = 0;
    while ( defined( my $file = $iter->() ) ) {
        my $mp3 = MP3::Tag->new($file);
        $mp3->get_tags;
        my $id3v2 = $mp3->{ID3v2}
            or next;

        # for my $frame ( sort keys $id3v2->get_frame_ids->%* ) {
        #     next if $frame eq 'APIC';
        #     my ($info, $name, @rest) =$id3v2->get_frame($frame);
        #     use Devel::Dwarn;
        #     Dwarn {
        #         $frame => { info => $info, name => $name, rest => \@rest } };
        # }

        my ($set) = $id3v2->get_frame('TPOS');
        if ( $set && $set !~ /^1/ ) {
            next;
        }

        my ($album)        = $id3v2->get_frame('TALB');
        my ($track_name)   = $id3v2->get_frame('TIT2');
        my ($release_date) = $id3v2->get_frame('TDOR');
        my ($artist)       = $id3v2->get_frame('TPE2');
        my ($track_num)    = ( split /\//, $id3v2->get_frame('TRCK') )[0];
        my ($length)       = $id3v2->get_frame('TLEN');

        next
            unless all {defined}
        $album, $track_name, $release_date, $artist, $track_num;

        $artists{$artist}{albums}{$album} = 1;

        $albums{$album} //= {
            album        => $album,
            artist       => $artist,
            release_date => $release_date,
        };
        $albums{$album}{tracks}[ $track_num - 1 ] = {
            title     => $track_name,
            track_num => $track_num,
        };

        # use Devel::Dwarn;
        # Dwarn $tags
        #     if $tags->{ARTIST} =~ /beat /i
        #     && $tags->{ARTIST} ne 'BEAT CRUSADERS';
    }

    return ( \%artists, \%albums );
}

package main;

exit Maker->new_with_options->run;
