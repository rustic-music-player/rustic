#include "bindings.h"
#include <stdio.h>

int main(void) {
    RusticClientHandle* client = connect_http_client("http://localhost:8080");
    const FFIAlbumModel* album = client_get_album_blocking(client, "Z211c2ljOmFsYnVtOkJqYXF3ZWlraHRianJuNnVoczMzcHJ3aGdidQ==");

    printf("Album { cursor: %s, title: %s, artist: %p, in_library: %d }\n", album->cursor, album->title, album->artist, album->in_library);

    return 0;
}
