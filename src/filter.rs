//! Filter builder - Construct your filter to filter out server results
//!
//! NOTE: Some filters may or may not work as expected depending on
//! appid/games you try it on. The filter builder methods and string
//! construction generally follows close to the reference listed out
//! in the Valve developer wiki.
//!
//! Reference: <https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol#Filter>
//!
//! # Quick Start
//!
//! ```
//! use msq::Filter;
//! let filter = Filter::new()
//!     .appid(240)
//!     .full(false)
//!     .map("de_dust2");
//! ```
//!
#[derive(Clone)]
enum FilterPropVal {
    Special(Vec<FilterProp>),
    Boolean(bool),
    Str(String),
    Uint32(u32),
    Tags(Vec<String>),
}

impl FilterPropVal {
    fn from_special(spec: &Vec<FilterProp>) -> FilterPropVal {
        Self::Special(spec.clone())
    }

    fn from_tags(tags: &Vec<&str>) -> FilterPropVal {
        let mut fpvtags: Vec<String> = vec![];

        for tag in tags {
            fpvtags.push(String::from(*tag));
        }

        Self::Tags(fpvtags)
    }

    fn as_str(&self) -> String {
        match &*self {
            Self::Special(filterprops) => {
                let mut sstr = String::from("");

                // Start with values count
                sstr += &format!("{}", filterprops.len());

                // Populate the string with inner values
                for fp in filterprops {
                    sstr += &fp.as_str();
                }

                sstr
            }
            Self::Boolean(b) => format!("{}", *b as i32),
            Self::Str(s) => String::from(s),
            Self::Uint32(i) => format!("{}", i),
            Self::Tags(tags) => {
                let mut tags_str = String::from("");
                for tag in tags {
                    tags_str += &tag;
                    tags_str += ",";
                }
                tags_str.pop();
                tags_str
            }
        }
    }
}

#[derive(Clone)]
struct FilterProp {
    pub name: String,
    pub value: FilterPropVal,
}

impl FilterProp {
    fn new(name: &str, value: FilterPropVal) -> FilterProp {
        FilterProp {
            name: String::from(name),
            value: value,
        }
    }

    fn as_str(&self) -> String {
        format!("\\{}\\{}", self.name, self.value.as_str())
    }
}

/// Filter builder - Construct your filter to filter out server results
///
/// NOTE: Some filters may or may not work as expected depending on
/// appid/games you try it on. The filter builder methods and string
/// construction generally follows close to the reference listed out
/// in the Valve developer wiki.
///
/// Reference: <https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol#Filter>
///
/// # Quick Start
///
/// ```
/// use msq::Filter;
/// let filter = Filter::new()
///     .appid(240)
///     .full(false)
///     .map("de_dust2");
/// ```
///
pub struct Filter {
    filter_lst: Vec<FilterProp>,
    in_special: bool,
    spec_vec: Vec<FilterProp>,
    special_name: String,
}

impl Filter {
    /// Returns a string representing the filters
    #[deprecated(since = "0.1.2", note = "Replaced with as_string (name change)")]
    pub fn as_str(&self) -> String {
        self.as_string()
    }

    /// Returns a string representing the filters
    pub fn as_string(&self) -> String {
        let mut sstr = String::from("");

        for fp in &self.filter_lst {
            sstr += &fp.as_str();
        }

        sstr
    }

    /// Returns a new Filter struct, used for string builder
    ///
    /// # Examples
    /// ```
    /// // Filter
    /// use msq::Filter;
    /// let filter = Filter::new()
    ///     .appid(240)
    ///     .full(false)
    ///     .map("de_dust2");
    /// ```
    pub fn new() -> Filter {
        Filter {
            filter_lst: vec![],
            in_special: false,
            spec_vec: vec![],
            special_name: String::from(""),
        }
    }

    fn push(mut self, name: &str, value: FilterPropVal) -> Filter {
        if self.in_special {
            self.spec_vec.push(FilterProp::new(name, value));
        } else {
            self.filter_lst.push(FilterProp::new(name, value));
        }
        self
    }

    // Generic filter: Boolean
    fn boolean(self, name: &str, switch: bool) -> Filter {
        self.push(name, FilterPropVal::Boolean(switch))
    }

    // Generic filter: String
    fn string(self, name: &str, param: &str) -> Filter {
        self.push(name, FilterPropVal::Str(String::from(param)))
    }

    // Generic filter: Unsigned integer of 32 bits
    fn uint32(self, name: &str, num: u32) -> Filter {
        self.push(name, FilterPropVal::Uint32(num))
    }

    // Generic filter: Vector of strings
    fn vecstr(self, name: &str, tags: &Vec<&str>) -> Filter {
        if tags.len() > 0 {
            self.push(name, FilterPropVal::from_tags(tags))
        } else {
            self
        }
    }

    // Generic filter: Special (start)
    fn special_start(mut self, name: &str) -> Filter {
        self.spec_vec.clear();
        self.in_special = true;
        self.special_name = String::from(name);
        self
    }

    /// A special filter, specifies that servers matching any of the following \[x\] conditions should not be returned
    /// See [pub fn end] method to see examples on usage
    pub fn nor(self) -> Filter {
        self.special_start("nor")
    }

    /// A special filter, specifies that servers matching all of the following \[x\] conditions should not be returned
    /// See [pub fn end] method to see examples on usage
    pub fn nand(self) -> Filter {
        self.special_start("nand")
    }

    /// End the special filter (nor, nand)
    /// You must use this method after each nor/nand special filter method being used
    ///
    /// # Examples
    /// Using the NAND filter:
    /// ```
    /// use msq::Filter;
    /// let filter = Filter::new()
    ///     .appid(240)
    ///     .nand()     // Exclude servers that has de_dust2 AND is empty
    ///         .map("de_dust2")
    ///         .empty(true)
    ///     .end()      // Ends the NAND special filter
    ///     .gametype(&vec!["friendlyfire", "alltalk"]);
    /// ```
    ///
    /// Using the NOR filter:
    /// ```
    /// use msq::Filter;
    /// let filter = Filter::new()
    ///     .appid(240)
    ///     .nor()      // Exclude servers that has de_dust2 OR is empty
    ///         .map("de_dust2")
    ///         .empty(true)
    ///     .end()      // Ends the NOR special filter
    ///     .gametype(&vec!["friendlyfire", "alltalk"]);
    /// ```
    pub fn end(mut self) -> Filter {
        self.filter_lst.push(FilterProp::new(
            &self.special_name,
            FilterPropVal::from_special(&self.spec_vec),
        ));
        self.in_special = false;
        self.special_name = String::from("");
        self
    }

    /// Filters if the servers running dedicated
    ///
    /// # Arguments
    /// * `is_dedicated` - A bool
    pub fn dedicated(self, is_dedicated: bool) -> Filter {
        self.boolean("dedicated", is_dedicated)
    }

    /// Servers using anti-cheat technology (VAC, but potentially others as well)
    pub fn secure(self, hasac: bool) -> Filter {
        self.boolean("secure", hasac)
    }

    /// Servers running the specified modification (ex: cstrike)
    pub fn gamedir(self, modg: &str) -> Filter {
        self.string("gamedir", modg)
    }

    /// Servers running the specified map (ex: cs_italy)
    pub fn map(self, mapn: &str) -> Filter {
        self.string("map", mapn)
    }

    /// Servers running on a Linux platform
    pub fn linux(self, runslinux: bool) -> Filter {
        self.boolean("linux", runslinux)
    }

    /// Servers that are not password protected
    pub fn password(self, protected: bool) -> Filter {
        self.boolean("password", protected)
    }

    /// Servers that are full
    pub fn full(self, is_full: bool) -> Filter {
        self.boolean("full", !is_full)
    }

    /// Servers that are spectator proxies
    pub fn proxy(self, specprox: bool) -> Filter {
        self.boolean("proxy", specprox)
    }

    /// Servers that are running game \[appid\]
    pub fn appid(self, appid: u32) -> Filter {
        self.uint32("appid", appid)
    }

    /// Servers that are NOT running game \[appid\]
    pub fn napp(self, appid: u32) -> Filter {
        self.uint32("napp", appid)
    }

    /// Servers that are empty: is_empty = true
    /// Servers that are not empty: is_empty = false
    pub fn empty(self, is_empty: bool) -> Filter {
        if is_empty {
            self.boolean("noplayers", true)
        } else {
            self.boolean("empty", true)
        }
    }

    /// Servers that are whitelisted
    pub fn whitelisted(self, white: bool) -> Filter {
        self.boolean("white", white)
    }

    /// Servers with all of the given tag(s) in sv_tags
    ///
    /// # Arguments
    /// * `tags` - A vector of strings which represents a tag from sv_tags
    ///
    /// # Example
    /// ```
    /// use msq::Filter;
    /// let filter = Filter::new()
    ///     .appid(240)
    ///     .gametype(&vec!["friendlyfire", "alltalk"]);
    /// ```
    ///
    /// If you put in an empty vector, it will return nothing
    pub fn gametype(self, tags: &Vec<&str>) -> Filter {
        self.vecstr("gametype", tags)
    }

    /// Servers with all of the given tag(s) in their 'hidden' tags (L4D2)
    /// # Arguments
    /// * `tags` - A vector of strings which represents a tag from sv_tags
    pub fn gamedata(self, tags: &Vec<&str>) -> Filter {
        self.vecstr("gamedata", tags)
    }

    /// Servers with any of the given tag(s) in their 'hidden' tags (L4D2)
    /// # Arguments
    /// * `tags` - A vector of strings which represents a tag from sv_tags
    pub fn gamedataor(self, tags: &Vec<&str>) -> Filter {
        self.vecstr("gamedataor", tags)
    }

    /// Servers with their hostname matching \[hostname\] (can use * as a wildcard)
    pub fn name_match(self, hostname: &str) -> Filter {
        self.string("name_match", hostname)
    }

    /// Servers running version \[version\] (can use * as a wildcard)
    pub fn version_match(self, ver: &str) -> Filter {
        self.string("version_match", ver)
    }

    /// Return only one server for each unique IP address matched
    pub fn collapse_addr_hash(self, one_server: bool) -> Filter {
        self.boolean("collapse_addr_hash", one_server)
    }

    /// Return only servers on the specified IP address (port supported and optional)
    pub fn gameaddr(self, ipaddr: &str) -> Filter {
        self.string("gameaddr", ipaddr)
    }
}
