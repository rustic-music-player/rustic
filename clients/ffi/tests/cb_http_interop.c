#include "bindings.h"
#include <stdio.h>

void callback(char* error, const FFIAlbumModel* album) {
    printf("Album { cursor: %s, title: %s, in_library: %d }\n", album->cursor, album->title, album->in_library);
}

int main(void) {
    RusticClientHandle* client = connect_http_client("http://localhost:8080");
    void (*ptr)() = &callback;
    client_get_album_cb(client, "ZmlsZTovLy9ob21lL21heC9NdXNpYy9taXh0YXBlL0ZvcnQgTWlub3IgLSBXZSBNYWpvciAoRGF0UGlmZi5jb20pLzAyIC0gMTAwIGRlZ3JlZXMubXAz", ptr);

    return 0;
}
