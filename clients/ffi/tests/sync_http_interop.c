#include "../bindings.h"

int main(void) {
    RusticClientHandle* client = connect_http_client("http://localhost:8080");
    const Player* player = client_get_default_player_sync(client);

    return 0;
}
