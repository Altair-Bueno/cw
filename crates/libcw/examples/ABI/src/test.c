#include "../lib/libcw.h"
#include "stdio.h"

#define EXAMPLE_FILE "example.txt"

void print_stats(Stats * stats) {
    printf("Bytes: %lu\n",stats->bytes);
    printf("Characters: %lu\n",stats->characters);
    printf("Lines: %lu\n",stats->lines);
    printf("Words: %lu\n",stats->words);
    printf("Length: %lu\n",stats->length);
}

int main(void) {
    // Configuration
    Encoding encoding = UTF8;
    LineBreak linebreak = LF;
    Parser * parser = new_parser(encoding,linebreak,true,true,true,true,true);
    Stats * stats = new_stats();
    int code;

    // Process a file
    code = process_file(parser,EXAMPLE_FILE,stats);
    printf("Process a file (code %i)\n",code);
    print_stats(stats);

    // Process a string
    char * string = "This is a string";
    code = process_string(parser,string,stats);
    printf("\nProcess a string (code %i)\n",code);
    print_stats(stats);

    // Process a slice
    int size = 1024;
    unsigned char * slice = (unsigned char *) malloc(sizeof(char) * size);
    FILE * file = fopen(EXAMPLE_FILE,"rt");
    int read = fread(slice,sizeof(unsigned char),size,file);

    code = process_slice(parser,slice,read,stats);
    printf("\nProcess a slice (code %i)\n",code);
    print_stats(stats);

    // Cleanup
    fclose(file);
    free(slice);
    destroy_stats(&stats);
    destroy_parser(&parser);
    return 0;
}