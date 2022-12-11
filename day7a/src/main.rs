use std::{cell::RefCell, path::PathBuf, rc::Rc, fs::read_to_string};

use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

#[derive(Debug, PartialEq, Eq)]
struct Command<'a> {
    name: &'a str,
    args: &'a str,
    results: &'a str,
}

impl Command<'_> {
    fn from_str(input: &str) -> Command {
        lazy_static! {
            static ref COMMAND_REGEX: Regex = RegexBuilder::new(r"\$ (\w+)( ([^\n]*))?(\n?(.*))")
                .dot_matches_new_line(true)
                .build()
                .unwrap();
        }

        let caps = COMMAND_REGEX.captures(input).unwrap();
        let name = match caps.get(1) {
            Some(m) => m.as_str().trim(),
            None => "",
        };

        let args = match caps.get(3) {
            Some(m) => m.as_str().trim(),
            None => "",
        };

        let results = match caps.get(5) {
            Some(m) => m.as_str().trim(),
            None => "",
        };

        Command {
            name,
            args,
            results,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum FileType {
    File,
    Directory,
}

#[derive(Debug, PartialEq, Eq)]
struct File<'a> {
    file_type: FileType,
    name: &'a str,
    size: usize,
    files: Vec<Rc<RefCell<File<'a>>>>,
}

impl<'a> File<'a> {
    fn new(file_type: FileType, name: &str, size: usize) -> File {
        File {
            file_type,
            name,
            size,
            files: Vec::<Rc<RefCell<File>>>::new(),
        }
    }

    fn set_files<'c, 'd>(&'c mut self, files: &'d mut Vec<Rc<RefCell<File<'a>>>>) {
        self.files.clear();
        self.files.append(files);
    }

    fn get<'b, 'c>(&'b self, name: &'c str) -> Option<&Rc<RefCell<File<'a>>>> {
        self.files.iter().find(|f| { 
            f.borrow().name == name 
        })
    }

    fn total_size(&self) -> usize {
        match self.file_type {
            FileType::File => self.size,
            FileType::Directory => self.files.iter().map(|f| f.borrow().total_size()).sum(),
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let sum = find_freeable_space(&input);
    println!("{}", sum);
}

fn find_freeable_space(input: &str) -> usize {
    // split into command strings
    let command_strings = split_into_command_strings(input);
    // parse commands
    let commands = parse_commands(&command_strings);
    // process commands
    let root = process_commands(&commands);

    // find all dirs with size < 100000
    sum_sizes_to_limit(&root, 100000)
}

fn sum_sizes_to_limit(root: &Rc<RefCell<File>>, limit: usize) -> usize {
    let mut queue = vec![root.clone()];
    let mut sum: usize = 0;
    loop {
        if queue.len() == 0 {
            break;
        }
        let current = queue.remove(0);
        let file = current.borrow();
        let size = file.total_size();
        if file.file_type == FileType::Directory && size <= limit {
            sum += size;
        }
        queue.append(&mut current.borrow().files.iter().cloned().collect::<Vec<Rc<RefCell<File>>>>());
    }

    sum
}

fn split_into_command_strings(input: &str) -> Vec<&str> {
    let mut result = Vec::<&str>::new();
    let command_regex = Regex::new(r"\$[^\$]*").unwrap();
    let matches = command_regex.find_iter(input);
    for m in matches {
        result.push(m.as_str().trim());
    }
    result
}

fn parse_commands<'a>(command_strings: &'a Vec<&str>) -> Vec<Command<'a>> {
    let mut commands = Vec::<Command>::with_capacity(command_strings.len());
    for cstr in command_strings.iter() {
        commands.push(Command::from_str(cstr));
    }
    commands
}

fn process_commands<'a, 'b>(commands: &'a Vec<Command<'a>>) -> Rc<RefCell<File<'b>>>
where
    'a: 'b,
{
    let mut path = PathBuf::from("/");
    let root = Rc::new(RefCell::new(File::new(FileType::Directory, "", 0)));
    let mut stack: Vec<Rc<RefCell<File>>> = vec![root.clone()];

    for cmd in commands.iter() {
        if cmd.name == "cd" {
            if cmd.args == "/" {
                stack.drain(1..);
            } else if cmd.args == ".." {
                stack.pop();
            } else {
                let current = stack.last().unwrap();
                let child = current.borrow().get(cmd.args).unwrap().clone();
                stack.push(child);
            }
            path.push(&cmd.args)
        } else if cmd.name == "ls" {
            let current = stack.last().unwrap();
            current.borrow_mut().set_files(&mut process_ls_results(&cmd.results));
        }
    }
    root
}

fn process_ls_results(cmd_output: &str) -> Vec<Rc<RefCell<File>>> {
    lazy_static! {
        static ref LS_REGEX: Regex = Regex::new(r"((dir)|(\d+)) (\S+)").unwrap();
    }

    let files = cmd_output
        .lines()
        .map(|l| {
            let caps = LS_REGEX.captures(l).unwrap();
            let file_type = match caps.get(2) {
                Some(_) => FileType::Directory,
                None => FileType::File,
            };

            let name = match caps.get(4) {
                Some(n) => n.as_str(),
                None => "",
            };

            let size: usize = match caps.get(3) {
                Some(n) => n.as_str().parse::<usize>().unwrap(),
                None => 0,
            };

            Rc::new(RefCell::new(File::new(file_type, name, size)))
        })
        .collect::<Vec<Rc<RefCell<File>>>>();

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn given_test_input_returns_95437() {
        let result = find_freeable_space(&TEST_INPUT.trim());
        assert_eq!(result, 95437);
    }

    #[test]
    fn given_test_input_splits_into_commands() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn given_test_input_splits_into_commands_cmd0() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ cd /
"
        .trim();
        assert_eq!(result.get(0).unwrap(), &cmd_string);
    }
    #[test]
    fn given_test_input_splits_into_commands_cmd1() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
"
        .trim();
        assert_eq!(result.get(1).unwrap(), &cmd_string);
    }

    #[test]
    fn given_test_input_splits_into_commands_cmd2() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ cd a
"
        .trim();
        assert_eq!(result.get(2).unwrap(), &cmd_string);
    }

    #[test]
    fn given_test_input_splits_into_commands_cmd3() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ ls
dir e
29116 f
2557 g
62596 h.lst
"
        .trim();
        assert_eq!(result.get(3).unwrap(), &cmd_string);
    }

    #[test]
    fn given_test_input_splits_into_commands_cmd4() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ cd e
"
        .trim();
        assert_eq!(result.get(4).unwrap(), &cmd_string);
    }

    #[test]
    fn given_test_input_splits_into_commands_cmd5() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ ls
584 i
"
        .trim();
        assert_eq!(result.get(5).unwrap(), &cmd_string);
    }

    #[test]
    fn given_test_input_splits_into_commands_cmd6() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ cd ..
"
        .trim();
        assert_eq!(result.get(6).unwrap(), &cmd_string);
    }

    #[test]
    fn given_test_input_splits_into_commands_cmd7() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ cd ..
"
        .trim();
        assert_eq!(result.get(7).unwrap(), &cmd_string);
    }

    #[test]
    fn given_test_input_splits_into_commands_cmd8() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ cd d
"
        .trim();
        assert_eq!(result.get(8).unwrap(), &cmd_string);
    }

    #[test]
    fn given_test_input_splits_into_commands_cmd9() {
        let result = split_into_command_strings(&TEST_INPUT.trim());
        let cmd_string = r"
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
        .trim();
        assert_eq!(result.get(9).unwrap(), &cmd_string);
    }

    #[test]
    fn given_cd_returns_correct_command() {
        let expected = Command {
            name: "cd",
            args: "/",
            results: "",
        };

        let input = "$ cd /";

        assert_eq!(Command::from_str(input), expected);
    }

    #[test]
    fn given_ls_returns_correct_command() {
        let expected = Command {
            name: "ls",
            args: "",
            results: r"4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        };

        let input = r"$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(Command::from_str(input), expected);
    }

    #[test]
    fn given_list_of_commands_returns_correct_vector() {
        let expected = vec![
            Command {
                name: "cd",
                args: "/",
                results: "",
            },
            Command {
                name: "ls",
                args: "",
                results: r"4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
            },
        ];

        let input = vec![
            "$ cd /",
            r"$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        ];
        let result = parse_commands(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn given_ls_output_returns_vec_files() {
        let input = r"4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let expected = vec![
            Rc::new(RefCell::new(File {
                file_type: FileType::File,
                name: "j",
                size: 4060174,
                files: vec![],
            })),
            Rc::new(RefCell::new(File {
                file_type: FileType::File,
                name: "d.log",
                size: 8033020,
                files: vec![],
            })),
            Rc::new(RefCell::new(File {
                file_type: FileType::File,
                name: "d.ext",
                size: 5626152,
                files: vec![],
            })),
            Rc::new(RefCell::new(File {
                file_type: FileType::File,
                name: "k",
                size: 7214296,
                files: vec![],
            })),
        ];

        assert_eq!(process_ls_results(input), expected);
    }

    #[test]
    fn given_ls_output_with_dir_returns_vec_files() {
        let input = r"4060174 j
dir foo";

        let expected = vec![
            Rc::new(RefCell::new(File {
                file_type: FileType::File,
                name: "j",
                size: 4060174,
                files: vec![],
            })),
            Rc::new(RefCell::new(File {
                file_type: FileType::Directory,
                name: "foo",
                size: 0,
                files: vec![],
            })),
        ];

        assert_eq!(process_ls_results(input), expected);
    }

    #[test]
    fn given_simple_vec_of_commands_returns_files() {
        let input = vec![
            Command {
                name: "cd",
                args: "/",
                results: "",
            },
            Command {
                name: "ls",
                args: "",
                results: r"4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
            },
        ];

        let expected = Rc::new(RefCell::new(File {
            file_type: FileType::Directory,
            name: "",
            size: 0,
            files: vec![
                Rc::new(RefCell::new(File {
                    file_type: FileType::File,
                    name: "j",
                    size: 4060174,
                    files: vec![],
                })),
                Rc::new(RefCell::new(File {
                    file_type: FileType::File,
                    name: "d.log",
                    size: 8033020,
                    files: vec![],
                })),
                Rc::new(RefCell::new(File {
                    file_type: FileType::File,
                    name: "d.ext",
                    size: 5626152,
                    files: vec![],
                })),
                Rc::new(RefCell::new(File {
                    file_type: FileType::File,
                    name: "k",
                    size: 7214296,
                    files: vec![],
                })),
                ],
        }));

        assert_eq!(process_commands(&input), expected);
    }

    #[test]
    fn given_nested_vec_of_commands_returns_files() {
        let input = vec![
            Command {
                name: "cd",
                args: "/",
                results: "",
            },
            Command {
                name: "ls",
                args: "",
                results: r"4060174 j
dir foo",
            },
            Command {
                name: "cd",
                args: "foo",
                results: "",
            },
            Command {
                name: "ls",
                args: "",
                results: r"1 bar 
2000 baz",
            },
        ];

        let expected = Rc::new(RefCell::new(File {
            file_type: FileType::Directory,
            name: "",
            size: 0,
            files: vec![
                Rc::new(RefCell::new(File {
                    file_type: FileType::File,
                    name: "j",
                    size: 4060174,
                    files: vec![],
                })),
                Rc::new(RefCell::new(File {
                    file_type: FileType::Directory,
                    name: "foo",
                    size: 0,
                    files: vec![
                        Rc::new(RefCell::new(File {
                            file_type: FileType::File,
                            name: "bar",
                            size: 1,
                            files: vec![],
                        })),
                        Rc::new(RefCell::new(File {
                            file_type: FileType::File,
                            name: "bar",
                            size: 2000,
                            files: vec![],
                        })),
                    ],
                })),
            ],
        }));


        let result = process_commands(&input);
        assert_eq!(result.borrow().name, expected.borrow().name);
        assert_eq!(result.borrow().files.len(), expected.borrow().files.len());
        assert_eq!(result.borrow().files[1].borrow().files.len(), expected.borrow().files[1].borrow().files.len());
    }
}
