#!/usr/bin/env perl

use v5.32;
use strict;
use warnings;
use feature 'signatures';

package Deployer;

use File::pushd qw( pushd );

sub run {
    my $dir = pushd('db');
    _run(
        'sqitch',    'deploy',
        '--db-name', 'music_player',
    );
    return 0;
}

sub _run (@cmd) {
    say "Running: @cmd"
        or die $!;
    system(@cmd);
}

package main;

exit Deployer::run();
