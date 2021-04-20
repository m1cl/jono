use irc::client::prelude::*;

enum Color {
    DEFAULT,
    BLACK,
    DARGRAY,
    RED,
    LIGHTRED,
    GREEN,
    LIGHTGREEN,
    BROWN,
    YELLOW,
    BLUE,
    LIGHTBLUE,
    MAGENTA,
    LIGHTMAGENTA,
    CYAN,
    LIGHTCYAN,
    GRAY,
    WHITE,
}

enum ColorOptionsName {
    SEPARATOR,
    CHAT,
    CHAT_TIME,
    CHAT_TIME_DELIMITERS,
    CHAT_PREFIX_ERROR,
    CHAT_PREFIX_NETWORK,
    CHAT_PREFIX_ACTION,
    CHAT_PREFIX_JOIN,
    CHAT_PREFIX_QUIT,
    CHAT_PREFIX_MORE,
    CHAT_PREFIX_SUFFIX,
    CHAT_BUFFER,
    CHAT_SERVER,
    CHAT_CHANNEL,
    CHAT_NICK,
    CHAT_NICK_SELF,
    CHAT_NICK_OTHER,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    INVALID,
    CHAT_HOST,
    CHAT_DELIMITERS,
    CHAT_HIGHLIGHT,
    CHAT_READ_MARKER,
    CHAT_TEXT_FOUND,
    CHAT_VALUE,
    CHAT_PREFIX_BUFFER,
    CHAT_TAGS,
    CHAT_INACTIVE_WINDOW,
    CHAT_INACTIVE_BUFFER,
    CHAT_PREFIX_BUFFER_INACTIVE_BUFFER,
    CHAT_NICK_OFFLINE,
    CHAT_NICK_OFFLINE_HIGHLIGHT,
    CHAT_NICK_PREFIX,
    CHAT_NICK_SUFFIX,
    EMPHASIS,
}

struct Message {}

struct Info {}

struct InfoList {}

struct NickList {}

struct HData {}

struct Weechat {
    config: Config,
}

impl Weechat {
    fn new(nickname: Option<String>, server: Option<String>) -> Weechat {
        let weechat = match nickname {
            Ok(nick) => {
                let config = Config {
                    nickname: nick,
                    ..Config::default()
                };
                Weechat { config }
            }
            Err(err) => println!("Something went wrong {:?}", err),
        };
        match server {
            Ok(server) => {
                weechat.server = server;
            }
            _ => weechat.server = "irc.freenode.org",
        }
        weechat
    }
    fn connect(host: String, port: u8) {}

    fn get_user_list() {}

    fn get_server_info() {}

    fn wee_handshake() {}

    fn wee_init() {}

    fn wee_sync() {}

    fn get_nicklist() -> NickList {}

    fn wee_ping() {}
}
