# Fragments assembly


## Generate synth with Simlord

    source simlord_env/bin/activate

    simlord --generate-reference 0.6 100000 --save-reference 1.dna.fasta -n 1000 --no-sam  1 -pi 0.0 -pd 0.0 -ps 0.00000001

after --no-sam put the the name of the fragments file
10M is the number of bases
0.6 is the percentage of CG
-n is the number of fragments
-pi proba insertion
-pd proba deletion
-ps proba sub

### CANU

    ./../../../canu-2.3/bin/canu -p name -d canu1 -genomeSize=100000 -pacbio 1.fastq 

requires cover >= 100

## Compare with Quast

    source simlord_env/bin/activate

    quast.py -r gene.dna.fasta  assembler.contigs.fasta -o resultsDirectory


