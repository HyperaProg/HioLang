/* 
 * HioClib Library Example - String Utils in C
 * This library provides string manipulation utilities
 * that can be used in Hiolang programs
 */

#include <string.h>
#include <stdlib.h>
#include <ctype.h>

/* Convert string to uppercase */
char* hio_string_to_upper(const char* input) {
    char* result = malloc(strlen(input) + 1);
    if (result == NULL) return NULL;
    
    for (int i = 0; input[i]; i++) {
        result[i] = toupper(input[i]);
    }
    result[strlen(input)] = '\0';
    return result;
}

/* Convert string to lowercase */
char* hio_string_to_lower(const char* input) {
    char* result = malloc(strlen(input) + 1);
    if (result == NULL) return NULL;
    
    for (int i = 0; input[i]; i++) {
        result[i] = tolower(input[i]);
    }
    result[strlen(input)] = '\0';
    return result;
}

/* Reverse a string */
char* hio_string_reverse(const char* input) {
    int len = strlen(input);
    char* result = malloc(len + 1);
    if (result == NULL) return NULL;
    
    for (int i = 0; i < len; i++) {
        result[i] = input[len - 1 - i];
    }
    result[len] = '\0';
    return result;
}

/* Repeat a string n times */
char* hio_string_repeat(const char* input, int count) {
    int len = strlen(input);
    char* result = malloc(len * count + 1);
    if (result == NULL) return NULL;
    
    result[0] = '\0';
    for (int i = 0; i < count; i++) {
        strcat(result, input);
    }
    return result;
}
