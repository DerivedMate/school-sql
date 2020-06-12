#!/usr/bin/perl
use warnings;
use strict;
use Text::Trim qw(trim);

sub bare_name {
    my $name = shift;

    my ($bare) = ($name =~ /([\w\d]+\.\w+)$/);
    $bare
}

my ($dir_in, $dir_out) = (shift, shift);

opendir (my $inh, $dir_in) or die "Failed to open dir \"$dir_in\": $!";
my @files = grep {not /^\./}readdir $inh;

foreach my $f (@files) {
    my $f_full = $dir_in =~ /\/$/ ? $dir_in.$f : "$dir_in/$f";
    my $f_out_full = $dir_out =~ /\/$/ ? $dir_out.$f : "$dir_out/$f";
    print STDERR "$f_out_full\n";
    
    open (my $fh, "<", $f_full) or die $!;
    open (my $outh, ">", $f_out_full) or die $!;

    foreach my $l (<$fh>) {
        $l =~ s/\t+/;/g;
        $l = trim $l;
        print {$outh} "$l\n";
    }

    close $fh;
    close $outh;
}

closedir $inh;