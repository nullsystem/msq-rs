pub struct Filter {
    filter_str: String,
}

// Filters list: https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol#Filter

impl Filter {
    pub fn as_str(&self) -> &str {
        return &self.filter_str;
    }

    pub fn new() -> Filter {
        Filter {
            filter_str: String::from(""),
        }
    }

    // Generic filter: Special
    fn special(mut self, name: &str) -> Filter {
        self.filter_str.push_str(&format!("\\{}\\", name));
        self
    }

    // Generic filter: Boolean
    fn boolean(mut self, name: &str, switch: bool) -> Filter {
        self.filter_str
            .push_str(&format!("\\{}\\{}", name, switch as i32));
        self
    }

    // Generic filter: String
    fn string(mut self, name: &str, param: &str) -> Filter {
        self.filter_str.push_str(&format!("\\{}\\{}", name, param));
        self
    }

    // Generic filter: Unsigned integer of 32 bits
    fn uint32(mut self, name: &str, num: u32) -> Filter {
        self.filter_str.push_str(&format!("\\{}\\{}", name, num));
        self
    }

    // Generic filter: Vector of strings
    fn vecstr(mut self, name: &str, tags: &Vec<&str>) -> Filter {
        if tags.len() > 0 {
            self.filter_str.push_str(&format!("\\{}\\", name));
            for tag in tags {
                self.filter_str.push_str(&format!("{},", tag));
            }
            self.filter_str.pop();
        }
        self
    }

    // A special filter, specifies that servers matching any of the following [x] conditions should not be returned
    pub fn nor(self) -> Filter {
        self.special("nor")
    }

    // A special filter, specifies that servers matching all of the following [x] conditions should not be returned
    pub fn nand(self) -> Filter {
        self.special("nand")
    }

    // Servers running dedicated
    pub fn dedicated(self, is_dedicated: bool) -> Filter {
        self.boolean("dedicated", is_dedicated)
    }

    // Servers using anti-cheat technology (VAC, but potentially others as well)
    pub fn secure(self, hasac: bool) -> Filter {
        self.boolean("secure", hasac)
    }

    // Servers running the specified modification (ex: cstrike)
    pub fn gamedir(self, modg: &str) -> Filter {
        self.string("gamedir", modg)
    }

    // Servers running the specified map (ex: cs_italy)
    pub fn map(self, mapn: &str) -> Filter {
        self.string("map", mapn)
    }

    // Servers running on a Linux platform
    pub fn linux(self, runslinux: bool) -> Filter {
        self.boolean("linux", runslinux)
    }

    // Servers that are not password protected
    pub fn password(self, protected: bool) -> Filter {
        self.boolean("password", protected)
    }

    // Servers that are full
    pub fn full(self, is_full: bool) -> Filter {
        self.boolean("full", !is_full)
    }

    // Servers that are spectator proxies
    pub fn proxy(self, specprox: bool) -> Filter {
        self.boolean("proxy", specprox)
    }

    // Servers that are running game [appid]
    pub fn appid(self, appid: u32) -> Filter {
        self.uint32("appid", appid)
    }

    // Servers that are NOT running game [appid]
    pub fn napp(self, appid: u32) -> Filter {
        self.uint32("napp", appid)
    }

    // Servers that are empty: is_empty = true
    // Servers that are not empty: is_empty = false
    pub fn empty(self, is_empty: bool) -> Filter {
        if is_empty {
            self.boolean("noplayers", true)
        } else {
            self.boolean("empty", true)
        }
    }

    // Servers that are whitelisted
    pub fn whitelisted(self, white: bool) -> Filter {
        self.boolean("white", white)
    }

    // Servers with all of the given tag(s) in sv_tags
    pub fn gametype(self, tags: &Vec<&str>) -> Filter {
        self.vecstr("gametype", tags)
    }

    // Servers with all of the given tag(s) in their 'hidden' tags (L4D2)
    pub fn gamedata(self, tags: &Vec<&str>) -> Filter {
        self.vecstr("gamedata", tags)
    }

    // Servers with any of the given tag(s) in their 'hidden' tags (L4D2)
    pub fn gamedataor(self, tags: &Vec<&str>) -> Filter {
        self.vecstr("gamedataor", tags)
    }

    // Servers with their hostname matching [hostname] (can use * as a wildcard)
    pub fn name_match(self, hostname: &str) -> Filter {
        self.string("name_match", hostname)
    }

    // Servers running version [version] (can use * as a wildcard)
    pub fn version_match(self, ver: &str) -> Filter {
        self.string("version_match", ver)
    }

    // Return only one server for each unique IP address matched
    pub fn collapse_addr_hash(self, one_server: bool) -> Filter {
        self.boolean("collapse_addr_hash", one_server)
    }

    // Return only servers on the specified IP address (port supported and optional)
    pub fn gameaddr(self, ipaddr: &str) -> Filter {
        self.string("gameaddr", ipaddr)
    }
}
