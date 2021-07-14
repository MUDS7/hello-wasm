1.解析文本格式文件，将需要解析的文件放在src目录下（其他目录也行）
  在main.rs的main函数中修改“include_bytes!("PIPE-100-B-1.rvm")”的文件名（或路径）
  再点击运行即可

2.解析二进制格式文件，解析二进制文件功能都放在了”parse_binary.rs“文件中，
  找到fn test_parse_binary()测试函数，修改 let file=include_bytes!("D:/test_binary.rvm");中的文件目录后
  点击运行即可。

3.项目中的对象已做过序列化，可将运行结果输出到文件中，若要输出到文件中就将
                                                            let mut s=String::new();
                                                            tree.write_formatted(&mut s).unwrap();
                                                            println!("s={}",s);
                                                            注释掉，再调用相应的io方法。