use serde::{Deserialize, Serialize};
// use serde_json::from_str;
use std::{ fs::{self, DirEntry, File, FileType},  io::{self, Read, Write}, path::Path, process::{ Output }, str };
// use windows::Win32::UI::Input::KeyboardAndMouse::{*};
use windows::Win32::{Foundation::*, Graphics::Gdi::{DISPLAY_DEVICEW, HMONITOR}, System::LibraryLoader::*, UI::{Input::KeyboardAndMouse::{ VkKeyScanW, GMMP_USE_DISPLAY_POINTS, VK_LBUTTON, VK_RBUTTON}, WindowsAndMessaging::*}};
use windows::core::{ s };
use windows::Win32::UI::Input::KeyboardAndMouse::{*};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Macro {
    app_name: String,
    app: Vec<App>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct App {
    app_value: String,
    website_open: bool,
    steps: Vec<Steps>
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Steps {
    name: String,
    code: u16,
    held: bool,
    sentence: String,
    time: u16,
    r#loop: u8
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Keys {
    keys:Vec<KeyCodesCsv>
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct KeyCodesCsv {
    name: String,
    windows: u16,
    ascii: u16
}
fn main() -> Result<(), std::io::Error> {
    // match std::env::current_exe() {
    //     Ok(exe_path) => println!("Path: {:?}",exe_path.parent().expect("Not found")),
    //     Err(e) => println!("ERROR: {e}")
    // };
    let current_directory:Result<Output, io::Error> = execute_command("cmd", &["/C","cd"]);
    let directory_files:Result<Output,std::io::Error> = execute_command("cmd", &["/C", "dir /b /a-d"]);
    println!("{:?}",directory_files);
    let mut output: Vec<u8> = Vec::new();
    let mut directory_location: &str = "";
    match directory_files {
        Ok(v) => println!("{:?}", v),
        _ => println!("Error")
    };
    match current_directory {
        Ok(v) => {
            output = v.stdout;
            directory_location = std::str::from_utf8(&output).unwrap();
            println!("62 {:?}",&directory_location)
        },
        _ => println!("Error")
    };
    println!("DIR LOCATION: {}\\macros",&directory_location.replace("\r\n", ""));
    let list_files:Result<Output, io::Error> = execute_command("cmd", &["/C",format!("ls {dir}\\macros",dir=&directory_location.replace("\r\n", "")).as_str(),"dir /b /a-d"]);
    println!("68 {:?}",list_files);
    
    let mut keys_file:File = File::open(format!("{dir}\\{file}.json",dir=&directory_location.replace("\r\n", ""), file="keys")).unwrap();
    let mut keys_buffer:String = String::new();
    let _ = keys_file.read_to_string(&mut keys_buffer);
    let keys_json:Keys = serde_json::from_str(&keys_buffer).expect("Unable to get data");
    // keys_json.keys.iter().for_each(|f| {
    //     println!("{}, {}", &f.name, &f.ascii);
    // });
    println!("{:?}",keys_json.keys[0].name);
    
    let mut file_name: String = String::new();
    println!("Enter file name");
    let _ = io::stdin().read_line(&mut file_name);
    
    println!("{:?}",format!("{dir}\\macros\\{file}.json",dir=&directory_location.replace("\r\n", ""), file=file_name.trim()));
    let mut file:File = File::open(format!("{dir}\\macros\\{file}.json",dir=&directory_location.replace("\r\n", ""), file=file_name.trim())).unwrap();
    println!("{:?}",file);
    let mut buffer:String = String::new();
    let _ = file.read_to_string(&mut buffer);
    let data:Macro = serde_json::from_str(&buffer).expect("Not found");
    println!("{:?}",&buffer);
    

    let app: Vec<App> = data.app;
    
    // let open_app: Result<Output, std::io::Error> = execute_command("cmd",&["/C",format!("start {data}", data = & data.app_name).as_str(), "https://github.com/"]);
    // let virtual_keys_vec:Vec<u16> = vec![0x5B,0x90,0x91,0x14];
    let mut hold_keys_vector_steps:Vec<Steps> = Vec::new();
    // let mut hold_keys_vector:Vec<u16> = Vec::new();
    let virtual_keys_vector:Vec<u16> = Vec::new();
    for app in app.into_iter() {
        if app.website_open {
            println!("OPENING WEBSITE");
            // let website_to_open = &app[0].app_value;
            let _ = execute_command("cmd", &["/C", "start chrome --new-window", &app.app_value]);
            for step in app.steps.into_iter() {
                // steps_vec.push(step);
                // match from_str::<u16>(&step.code) {
                //     Ok(o) => virtual_keys_vector.push(o),
                //     _ => println!("ERROR")
                // };
                hold_keys_vector_steps.push(step);
                // if step.held {
                // }
                
                // virtual_keys_vector.push(step.code);
            }
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }else {
            println!("Opening {}", app.app_value);
            let file_to_open:String = format!("{}.exe",&app.app_value);
            let _ = execute_command("cmd", &["/C", "start", &file_to_open]);
            for step in app.steps.into_iter() {
                // steps_vec.push(step);
                // match from_str::<u16>(&step.code) {
                //     Ok(o) => virtual_keys_vector.push(o),
                //     _ => println!("ERROR")
                // };
                hold_keys_vector_steps.push(step);
                // if step.held {
                // }
                
                // virtual_keys_vector.push(step.code);
            }
            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    }
    virtual_keys_vector.iter().for_each(|x| {
        println!("{:?}", &x);
    });
    unsafe {
        // let instance = GetModuleHandleA(None).unwrap();
        // debug_assert!(instance.0 != 0);
        /*
        let window_class = s!("window");
        let window_class_a = WNDCLASSA {
            style: CS_OWNDC | CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: instance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            lpszClassName: window_class,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: Default::default(),
            hbrBackground: Default::default(),
            lpszMenuName: s!("Menu")
        };
        let atom:u16 = RegisterClassA(&window_class_a);
        debug_assert!(atom != 0);
        println!("Atom: {:?}",atom);
        let mut message:MSG = MSG::default();
        */
        let mut current_window:HWND = GetForegroundWindow();
        // PREVIOUSLY USED
        // let length:i32 = GetWindowTextLengthA(current_window) + 1;
        // let mut window_text: Vec<u16> = vec![];
        // GetWindowTextW(current_window,&mut window_text);
        // let mut client_rect: RECT = RECT {..Default::default()};
        let client_rect: RECT = RECT {
            left: 0,
            top: 0,
            right: 800,
            bottom: 800
        };
        // let current_window_opened = GetClientRect(current_window,&mut client_rect);
        // PREVIOUSLY USED ABOVE
        let _ = SetWindowPos(current_window,HWND_TOP,0,0,client_rect.right,client_rect.bottom,SWP_SHOWWINDOW);
        // let windows_position = MapWindowPoints(0, current_window,0 );
        // PREVIOUSLY USED
        // let mut text:Vec<u16> = vec![0;length as usize];
        // let _ = GetWindowTextW(current_window,&mut text);
        // PREVIOUSLY USED ABOVE
        println!("{:?}",current_window);
        let mut focus_rect:RECT = RECT {..Default::default()};
        let _ = SetForegroundWindow(current_window);
        let _ = SetActiveWindow(current_window);
        let _ = SetFocus(current_window);
        let _ = GetWindowRect(current_window, &mut focus_rect);
        let _ = SetCursorPos(focus_rect.right - 100, focus_rect.bottom - 100);
        // println!("line 109: {:?}",String::from_utf16_lossy(&text[..length as usize]));
        // send_input_messages(164, false);
        // std::thread::sleep(std::time::Duration::from_millis(1000));
        // send_input_messages(9, false);
        // std::thread::sleep(std::time::Duration::from_millis(5000));
        // send_input_messages(164, true);
        // send_input_messages(9, true);
        // std::thread::sleep(std::time::Duration::from_millis(2000));
        get_mouse_events();

        let mut holding_keys_to_release:Vec<u16> = Vec::new();
        let mut mouse_movements:Vec<Steps> = Vec::new();
        for key in hold_keys_vector_steps.iter() {
            println!("{:?}", key.name);
            
            for _ in 0..key.r#loop {
                // println!("{}", i);
                std::thread::sleep(std::time::Duration::from_millis(100));
                if key.held {
                    send_input_messages(key.code, false, true);
                    holding_keys_to_release.push(key.code);
                }else if key.code == 999 {
                    std::thread::sleep(std::time::Duration::from_secs(key.time.into()));
                }else if key.code > 800 && key.code < 900 {
                    println!("Mouse event");
                    mouse_movements.push({
                        Steps {
                            name: key.name.to_string(),
                            code: key.code,
                            sentence: key.sentence.to_string(),
                            held: key.held,
                            r#loop: key.r#loop,
                            time: key.time
                        }
                    })
                }else if key.code == 998 {
                    // println!("{:?}", key.sentence.split_ascii_whitespace().into_iter());
                    // let keysData:Keys = serde_json::from_str(&keysBuffer).expect("Not found");
                    key.sentence.as_bytes().into_iter().for_each(|f| {
                        // let u16_item = *f as u16;
                        // println!("{:?}",&u16_item);
                        let mut u16_total_key:u16 = 0;
                        let mut hex_code:String = format!("{f:#X}");
                        // let u16_hex = hex_code.as_bytes()[0];
                        // let format_hex_code:String = format!("{}",hex_code);
                        // hex_code = hex_code.replace("\"","");
                        hex_code = hex_code.replace("0x","");
                        let first_char:String = hex_code[..1].to_owned();
                        let second_char:String = hex_code[1..].to_owned();
                        // println!("{}, {}",first_char, second_char);
                        u16_total_key = first_char.parse::<u16>().unwrap();
                        u16_total_key = u16_total_key * 16;
                        if second_char == "A" {
                            u16_total_key = u16_total_key + 10;
                        }else if second_char == "B" {
                            u16_total_key = u16_total_key + 11;
                        }else if second_char == "C" {
                            u16_total_key = u16_total_key + 12;
                        }else if second_char == "D" {
                            u16_total_key = u16_total_key + 13;
                        }else if second_char == "E" {
                            u16_total_key = u16_total_key + 14;
                        }else if second_char == "F" {
                            u16_total_key = u16_total_key + 15;
                        }else {
                            u16_total_key = u16_total_key + second_char.parse::<u16>().unwrap()
                        }
                        let find_key = keys_json.keys.iter().find(|f| {
                            &f.ascii == &u16_total_key
                        });
                        // let u16_num = u16::from_str_radix(hex_code.as_str(),16);
                        // let mut number:u16 = 0;
                        // match u16_num {
                        //     Ok(v) =>number=v,
                        //     _ => println!("ERROR")
                        // }
                        let mut key_from_json:u16 = 0;
                        let mut key_char:&str = "";
                        match find_key {
                            Some(val) => {
                                // println!("{}, {}", val.ascii,val.name);
                                key_char = val.name.as_str();
                                key_from_json = val.ascii
                            },
                            None => println!("ERROR")
                        };
                        // check if key is less than u16 then shift
                        let key_from_json = VkKeyScanW(key_from_json);
                        // println!("f: {}, Key from Json: {}, total key: {}, hex code: {}",f,key_from_json,u16_total_key,hex_code);
                        // match second_char {
                        //     "A" => u16_total_key = u16_total_key + 10,
                        //     "B" => u16_total_key = u16_total_key + 11,
                        //     "C" => u16_total_key = u16_total_key + 12,
                        //     "D" => u16_total_key = u16_total_key + 13,
                        //     "E" => u16_total_key = u16_total_key + 14,
                        //     "F" => u16_total_key = u16_total_key + 15,
                        //     _ => u16_total_key = u16_total_key + second_char.parse::<u16>().unwrap(),
                        // }
                        // let character:u8 = format_hex_code.as_bytes()[format_hex_code.len() - 1];
                        // println!("{:?}",u16_total_key);
                        // println!("{:?}",hex_code.parse::<u16>().unwrap());
                        
                        // println!("{:?}",*f as u16);
                        // println!("HERE {}, {}, char: {}", (key_from_json >> 8 & 1) == 1, key_from_json,&key_char);
                        // if (key_from_json >> 8 & 1) == 1 {
                        //     send_input_messages(20, false, true);
                        // }
                        if (key_from_json >> 8 & 1) == 1 {
                            // let mut shift_key_state:i16 = GetKeyState(20);
                            send_input_messages(20, true, true);
                            std::thread::sleep(std::time::Duration::from_millis(100));
                            send_input_messages_from_i16(key_from_json, true, true);
                            // shift_key_state = GetKeyState(20);
                            std::thread::sleep(std::time::Duration::from_millis(100));
                            // println!("SHIFT STATE SHOULD BE 1 part 2: {:?}", shift_key_state);
                            send_input_messages(20, true, true);
                            // shift_key_state = GetKeyState(20);
                            // println!("SHIFT STATE SHOULD BE 1 part 3: {:?}", shift_key_state);

                        }else {
                            // let shift_key_state:i16 = GetKeyState(20);
                            // println!("HERE!!!! SHIFT STATE SHOULD BE 0 part 4: {:?}, {:?}, {}", shift_key_state,GetKeyState(20),key_from_json);
                            send_input_messages_from_i16(key_from_json, true, true);
                            
                        }
                        
                        
                        // send_input_messages_from_i16(key_from_json, true, true);
                        // send_input_messages(u16_num, true, true);
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    });
                    // key.sentence.chars().for_each(|f| {
                    //     // let u16_item = *f as u16;
                    //     // println!("{:?}",&u16_item);
                    //     // let hex_code:String = format!("{f:#X}");
                    //     // // let u16_hex = hex_code.as_bytes()[0];
                    //     // let u16_num = u16::from_str_radix(hex_code.as_str(), 16);
                    //     // let format_data = format!("{}",hex_code);
                    //     // println!("{:?}",hex_code.replace("\"", ""));
                    //     // println!("{:?}",*f as u16);
                    //     let u16_val: u16 = return_letter_number_from_u8(String::from(f),serde_json::from_str(&keysBuffer).expect("Not found"));
                    //     println!("{:?}", u16_val);
                    //     send_input_messages(u16_val, true, true);
                    //     // send_input_messages(u16_num, true, true);
                    //     std::thread::sleep(std::time::Duration::from_millis(250));
                    // });
                }else {
                    send_input_messages(key.code, true, true);
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
        mouse_movements.iter().for_each(|f| {
            if f.name.contains("move") {
                let word_split = f.sentence.split(",");
                let mut mouse_coords:[i32;2] = [0;2];
                let mut count = 0;
                for word in word_split {
                    // println!("{:?}", &word);
                    mouse_coords[count] = word.parse::<i32>().expect("Error");
                    count += 1
                };
                send_mouse_input_message(mouse_coords[0],mouse_coords[1]); 
            }
        });
        for key in holding_keys_to_release.iter() {
            send_input_messages(*key, true, false);
        }
        let mut keyboard_state_vec:[u8; 256] = [0;256];
        current_window = GetForegroundWindow();
        let mut window_rect: RECT = RECT { ..Default::default() };
        let _ = GetWindowRect(current_window, &mut window_rect);

        println!("{:?}", window_rect);
        send_mouse_input_message(100,100); 
        let primary_monitor:HMONITOR = windows::Win32::Graphics::Gdi::MonitorFromWindow(GetDesktopWindow(),windows::Win32::Graphics::Gdi::MONITOR_DEFAULTTOPRIMARY);
        let mut display_device_struct:DISPLAY_DEVICEW = DISPLAY_DEVICEW {
            ..Default::default()
        };
        // let _ = windows::Win32::Graphics::Gdi::EnumDisplayDevicesW(None,0,&mut display_device_struct,EDD_GET_DEVICE_INTERFACE_NAME);
        // let _ = GetKeyboardState(&mut keyboard_state_vec);
        println!("{:?}", primary_monitor);
        keyboard_state_vec.iter_mut().for_each(|f| {
            *f = 0
        });
        get_mouse_events();
        // let keyboard_state = GetKeyboardState(&mut keyboard_state_vec);
        // let _ = GetKeyboardState(&mut keyboard_state_vec);
        // println!("{:?}", keyboard_state_vec);
        // println!("{:?}", keyboard_state);
        // for item in virtual_keys_vector.iter() {
            
        //     send_input_messages(*item, true);
        //     std::thread::sleep(std::time::Duration::from_millis(2000));
        // }
        
        // send_input_messages(0x90);
        // GetMessageA(&mut message, None, 0,0);
        // while GetMessageA(&mut message, None, 0, 0).into(){
        //     DispatchMessageA(&message);
        // }
    }
    // Ok(());
    std::process::exit(0x000)
}
fn send_input_messages_from_i16(virtual_key_num:i16,release_key:bool, individial_press:bool) {
    unsafe {

        let get_key_state_int = virtual_key_num as u16;
        
        let input_zero: INPUT_0 = INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(get_key_state_int),
                wScan: get_key_state_int,
                dwFlags:KEYBD_EVENT_FLAGS(0x0001),
                time: 0,
                dwExtraInfo: 0x0008 as usize
            },
        };
        let release_zero: INPUT_0 = INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(get_key_state_int),
                wScan: get_key_state_int,
                dwFlags:KEYBD_EVENT_FLAGS(0x0002),
                time: 0,
                dwExtraInfo:0x0008 as usize
            },
        };
        let input_struct:INPUT = INPUT {
            r#type: INPUT_TYPE(1),
            Anonymous: input_zero
        };
        let input_release_struct:INPUT = INPUT{
            r#type: INPUT_TYPE(1),
            Anonymous: release_zero
        };
        let struct_size:i32 = core::mem::size_of::<INPUT>() as i32;
        // let _ = GetKeyState(get_key_state_int);
        if individial_press {
            // println!("{:?}", key_state);
            if release_key {
                let _ = SendInput(&[input_release_struct],struct_size);
            }
            let _ = SendInput(&[input_struct],struct_size);
        }else {
            let _ = SendInput(&[input_release_struct],struct_size);
        }
        // println!("{:?}", key_state);
    }
}

fn send_input_messages(virtual_key_num:u16, release_key:bool, individial_press:bool) {
    unsafe {

        // let get_key_state_int = virtual_key_num as i32;
        
        let input_zero: INPUT_0 = INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(virtual_key_num),
                wScan: virtual_key_num,
                dwFlags:KEYBD_EVENT_FLAGS(0x0001),
                time: 0,
                dwExtraInfo: 0x0008 as usize
            },
        };
        let release_zero: INPUT_0 = INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(virtual_key_num),
                wScan: virtual_key_num,
                dwFlags:KEYBD_EVENT_FLAGS(0x0002),
                time: 0,
                dwExtraInfo:0x0008 as usize
            },
        };
        let input_struct:INPUT = INPUT {
            r#type: INPUT_TYPE(1),
            Anonymous: input_zero
        };
        let input_release_struct:INPUT = INPUT{
            r#type: INPUT_TYPE(1),
            Anonymous: release_zero
        };
        let struct_size:i32 = core::mem::size_of::<INPUT>() as i32;
        // let _ = GetKeyState(get_key_state_int);
        if individial_press {
            let _ = SendInput(&[input_struct],struct_size);
            if release_key {
                let _ = SendInput(&[input_release_struct],struct_size);
            }
            // println!("{:?}", key_state);
        }else {
            let _ = SendInput(&[input_release_struct],struct_size);
        }
        // println!("{:?}", key_state);
    }
}

fn execute_command(exe: &str, args: &[&str]) -> Result<Output, std::io::Error>
{
    // let command:Output = Command::new(exe).args(&*args).output().expect("Can't run");
    std::process::Command::new(exe).args(&*args).output()
}
fn copy_all_files_in_directory(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()>
{
    fs::create_dir(&destination)?;
    for entry in fs::read_dir(&source)?
    {
        let entry:DirEntry = entry?;
        let try_get_file_type:FileType = entry.file_type()?;
        if try_get_file_type.is_dir()
        {
            copy_all_files_in_directory(entry.path(), &destination.as_ref().join(entry.file_name()))?;
        }else {
            fs::copy(entry.path(), &destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
fn get_mouse_events() {
    unsafe {
        const MOUSE_MOVE_POINT_STRUCT_CONST:MOUSEMOVEPOINT = MOUSEMOVEPOINT {
            x: 0 & 0x0000FFFF,
            y: 0 & 0x0000FFFF,
            time: 64,
            dwExtraInfo: 0x01
        };
        let mut mouse_move_point_struct:MOUSEMOVEPOINT = MOUSEMOVEPOINT {
            x: 0 & 0x0000FFFF,
            y: 0 & 0x0000FFFF,
            time: 64,
            dwExtraInfo: 0x01
        };
        let mouse_move_points = GetMouseMovePointsEx(core::mem::size_of::<MOUSEMOVEPOINT>() as u32,&MOUSE_MOVE_POINT_STRUCT_CONST,&mut [mouse_move_point_struct],GMMP_USE_DISPLAY_POINTS);
        println!("{:?}",&mouse_move_point_struct);
        println!("{:?}",&mouse_move_points);
    }
}
fn send_mouse_input_message(x:i32, y:i32) {
    println!("MOUSE EVENT");
    unsafe {
        // let mut point_struct:POINT = POINT {
        //     x:x,
        //     y:y
        // };
        // let _ = GetCursorPos(&mut point_struct);
        // println!("{:?}", point_struct);
        let input_mouse_struct:INPUT = INPUT {
            r#type: INPUT_TYPE(0),
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: x,
                    dy: y,
                    mouseData: WHEEL_DELTA,
                    dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_VIRTUALDESK ,
                    time: 0,
                    dwExtraInfo: Default::default()
                },
            }
        };
        let _ = SendInput(&[input_mouse_struct],core::mem::size_of::<INPUT>() as i32);
    }
}
/* extern "system" fn wnd_proc(window:HWND, message:u32, wparam:WPARAM, lparam:LPARAM) -> LRESULT
{
    // println!("Message: {:?}", message);
    unsafe {
       //  h_result:HRESULT = CreateGraphicsResources();
       
    //    let _ = BlockInput(true);
        match message {
            WM_ACTIVATEAPP => {
                println!("Active app: {:?}", message);
                LRESULT(0)
            },
            WM_CREATE => {
                println!("Create App: {:?}", message);
                
                let _ = CreateWindowExA(Default::default(),s!("STATIC"), s!("Test message"),WS_VISIBLE | WS_CHILD, 20,20, 300, 20, window,HMENU(1),None,None);
                let _ = CreateWindowExA(Default::default(), s!("BUTTON"), s!("Text inside"), WS_VISIBLE | WS_CHILD| WS_BORDER, 20, 50, 200, 20, window, HMENU(2), None, None);
                let _ = CreateWindowExA(Default::default(), s!("BUTTON"), s!("Close"), WS_VISIBLE | WS_CHILD| WS_BORDER, 20, 150, 200, 20, window, HMENU(3), None, None);
                let _ = CreateWindowExA(Default::default(), s!("EDIT"), s!("Hello World"),WS_VISIBLE | WS_CHILD| WS_BORDER, 30, 180, 200, 50, window, None,None, None);

                // let _ = BlockInput(false);
                LRESULT(0)
            },
            WM_COMMAND => {
                // let code_val = wparam.0 as i32;
                match wparam.0 as i32 {
                    1 => {
                        MessageBoxExA(window, s!("Button Pressed"), s!("BUTTON PRESSED"), MB_OK, Default::default());
                    },
                    2 => {
                        println!("2");
                    },
                    3 => {
                        PostQuitMessage(0);
                    },
                    4 => {
                        let mut text:Vec<u8>= vec![];
                        let val:i32 = GetWindowTextA(window, &mut text);
                        println!("199: {:?}", val);
                    },
                    _ => {
                        println!("Error");
                        let mut buffer: Vec<u16> = vec![0; (100 + 1) as usize];
                        // let mut text:Vec<u8>= vec![];
                        let val:i32 = GetWindowTextW(window, &mut buffer);
                        let window_text = String::from_utf16_lossy(&buffer[..100 as usize]);

                        println!("Active window text: {}", window_text);
                    }
                }
                println!("189: {:?}", wparam);

                // let _ = BlockInput(false);
                LRESULT(0)
            },
            // WM_PAINT => {
            //     println!("Paint APP: {:?}", message);
            //     // PAINTSTRUCT { hdc: val, fErase: val, rcPaint: val, fRestore: val, fIncUpdate: val, rgbReserved: val }
            //     let mut paint_struct:PAINTSTRUCT = PAINTSTRUCT { ..Default::default() };
            //     let hdc:HDC = BeginPaint(window, &mut paint_struct);
            //     // let rect:RECT = RECT {left:0, top:0, right:X_SIZE,bottom:Y_SIZE};
            //     // println!("RECT: {:?}",&rect);
            //     // let _ = ValidateRect(window, None);
                
            //     let mut client_rect: RECT = RECT {..Default::default()};
            //     let _ = GetClientRect(window,&mut client_rect);
            //     let mut original_object:HGDIOBJ = HGDIOBJ(0);
            //     original_object = SelectObject(paint_struct.hdc, GetStockObject(DC_PEN));
            //     let black_pen:HPEN = CreatePen(PS_SOLID, 3,COLORREF(0x000FFAA1));
            //     SelectObject(paint_struct.hdc, black_pen);
            //     let _ = Rectangle(paint_struct.hdc, client_rect.left + 100, client_rect.top + 100, client_rect.right - 100, client_rect.bottom - 100);

            //     let numbers:Vec<u16> = vec![67, 117, 114, 114, 101, 110, 116, 72, 111, 114, 105, 122, 111, 110, 116, 97, 108, 82, 101, 115, 111, 108, 117, 116, 105, 111, 110, 32, 32, 67, 117, 114, 114, 101, 110, 116, 86, 101, 114, 116, 105, 99, 97, 108, 82, 101, 115, 111, 108, 117, 116, 105, 111, 110, 32, 32, 13, 13, 10, 50, 53, 54, 48];
            //     // let msg: Vec<u8> = b"Peace!".to_vec();#
            //     let _ = Ellipse(hdc, 0, 100, 400, 400);
            //     // CreateEllipseGeometry()
            //     let mut rect: RECT = RECT {left:0, top:0, right:100,bottom:100};

            //     let _ = TextOutW(hdc, 0, 100, &numbers);
            //     // DrawTextExA(hdc, &mut msg, &mut rect, DT_LEFT | DT_TOP, None);
            //     FillRect(hdc, &mut rect, CreateSolidBrush(COLORREF(0x00A12345)));
            //     let delete:BOOL = DeleteObject(black_pen);
            //     let _ = EndPaint(window, &paint_struct);
            //     LRESULT(0)
            // },
            WM_CLOSE => {
                println!("Close APP: {:?}", message);
                PostQuitMessage(0);
                // let _ = BlockInput(false);
                LRESULT(0)
            },
            WM_DESTROY => {
                println!("Destoryed: {:?}", message);
                PostQuitMessage(0);
                // let _ = BlockInput(false);
                LRESULT(0)
            },
            _ => DefWindowProcA(window, message, wparam, lparam)
        }
    }
}
*/