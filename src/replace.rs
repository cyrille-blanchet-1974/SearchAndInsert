use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_search(
    from_read: Receiver<String>,
    to_write: Sender<String>,
    search_str: &str,
    replace_str: &str,
    only_first: bool,
    after: bool,
    exact_cmp: bool,
) -> JoinHandle<()> {
    let str_search = String::from(search_str);
    let str_replace = String::from(replace_str);
    let first_only = only_first;
    spawn(move || {
        let mut already_find = false;
        for l in from_read {
            let found = if exact_cmp {
                l == str_search
            } else {
                l.contains(&str_search)
            };
            let res: String = if found {
                if !already_find || !first_only {
                    already_find = true;
                    //insert or append
                    if after {
                        //write line found
                        if to_write.send(l).is_err() {
                            println!("error sending to write");
                            return;
                        }
                        //then the one to add
                        String::from(&str_replace)
                    } else {
                        //write line to insert first
                        if to_write.send(String::from(&str_replace)).is_err() {
                            println!("error sending to write");
                            return;
                        }
                        //then the one to add
                        l
                    }
                } else {
                    already_find = true;
                    l
                }
            } else {
                l
            };
            if to_write.send(res).is_err() {
                println!("error sending to write");
                return;
            }
        }
    })
}
