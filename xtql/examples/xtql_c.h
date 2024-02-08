#ifndef __XTQL_C_H_INCLUDED__
#define __XTQL_C_H_INCLUDED__

#ifdef __cplusplus
extern "C" {
#endif

/* parse an XTQL query and return a string for a valid JSON" 
 * This string needs to be freed */
const char* xtql_json_c(const char* s);
void free_rust_string(const char* s);


#ifdef __cplusplus
}
#endif


#endif  // __XTQL_C_H_INCLUDED__
