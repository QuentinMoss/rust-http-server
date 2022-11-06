use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// example: a=1&b=2&c&d=&e===&d=7&d=abc
// 'a=' -> key:a, val:1
// 'd=' -> key:d, val:["", 7, "abc"]
// 'e=' -> key:e, val:"=="

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        // Split on '&', pull key (a) and val (1)
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            // Option 'Some' or 'None', we only care about when the value is found, so use 'if let'
            // to only return when found
            if let Some(i) = sub_str.find('=') {
                // if = is found, assign the value before = to key
                key = &sub_str[..i];
                // then assin the value after = to val.
                // add padding of 1 to exclude the = from val
                // this match is inclusive
                val = &sub_str[i + 1..];
            }

            // First check hasmap for existing key
            // if the key already exists, we need to handle duplicate values:
            //
            //
            data.entry(key)
                // Closure (lambda)
                // 'existing' is a reference, a pointer, to Value
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        // Follow the pointer and write new value over previous pointer.
                        *existing = Value::Multiple(vec![prev_val, val])
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data };
        unimplemented!()
    }
}
