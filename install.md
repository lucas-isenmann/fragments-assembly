# Fragments assembly



## Canu assembler

Install at https://github.com/marbl/canu/releases

Install Samtools at https://www.htslib.org/download/ or sudo apt install samtools

If error: 

    Can't exec "command": No such file or directory at /adhome/l/lu/lucasisenmann/lab/FA_project/canu-2.3/bin/../lib/perl5/site_perl/canu/Defaults.pm line 1078.
    readline() on closed filehandle F at /adhome/l/lu/lucasisenmann/lab/FA_project/canu-2.3/bin/../lib/perl5/site_perl/canu/Defaults.pm line 1096.

Replace in lib/perl5/site_perl/canu/Defaults.pm at line 1096 

    # open(F, "$java -Xmx1g -showversion 2>&1 |");
    # @javaVersionStrings = <F>;
    # chomp @javaVersionStrings;
    # close(F);

by

    @javaVersionStrings = 'openjdk version "21.0.6" 2025-01-21\n OpenJDK Runtime Environment (build 21.0.6+7-Ubuntu-124.04.1) \nOpenJDK 64-Bit Server VM (build 21.0.6+7-Ubuntu-124.04.1, mixed mode, sharing)';
    chomp @javaVersionStrings;

or replace javaVersionStrings by the result of

    java -version




## Compare with Quast

Install https://pmc.ncbi.nlm.nih.gov/articles/PMC3624806/




