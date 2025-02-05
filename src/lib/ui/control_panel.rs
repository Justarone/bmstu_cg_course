use super::*;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
        }
    );
}

pub fn setup_control_panel(
    builder: &gtk::Builder,
    controller: &Arc<Mutex<Controller>>,
    drawing_area: &gtk::DrawingArea,
) {
    for (btn_name, key) in constants::COMMANDS_BUTTONS
        .iter()
        .zip(constants::CMDS_BTNS_KEY_MAP.iter())
    {
        let btn: gtk::Button = builder
            .get_object(btn_name)
            .expect(&format!("get {} object", btn_name));
        btn.connect_clicked(clone!(controller, drawing_area, key => move |_| {
            process_key(&controller, &drawing_area, key);
        }));
    }
    let mut inputs: Vec<gtk::Entry> = Vec::with_capacity(constants::INPUTS_AMOUNT);
    for inp_name in constants::INPUTS_NAMES.iter() {
        inputs.push(builder.get_object(inp_name).unwrap());
    }
    let mut rbtns: Vec<gtk::Button> = Vec::with_capacity(constants::RBTNS_AMOUNT);
    for rbtn_name in constants::RBTNS_NAMES.iter() {
        rbtns.push(builder.get_object(rbtn_name).unwrap());
    }

    setup_add(&rbtns, &inputs, &controller, &drawing_area);
    setup_mod(&rbtns, &inputs, &controller, &drawing_area);
    setup_del(&rbtns, &inputs, &controller, &drawing_area);
    setup_rpm(&rbtns, &inputs, &controller, &drawing_area);
    setup_next_prev(&rbtns, &inputs);
    setup_light_button(&rbtns, &inputs, &controller, &drawing_area);
}

fn setup_add(
    rbtns: &Vec<gtk::Button>,
    inputs: &Vec<gtk::Entry>,
    controller: &Arc<Mutex<Controller>>,
    drawing_area: &gtk::DrawingArea,
) {
    rbtns[constants::ADD_BTN].connect_clicked(
        clone!(inputs, controller, drawing_area => move |_| {
            let (pos, rad, gm) = match parse_all(&inputs) {
                Ok(res) => res,
                Err(_) => return,
            };
            {
                let mut controller = controller.lock().unwrap();
                if let Err(text) =
                    controller.restruct_muscle(MuscleOperation::Add(MOParams::new(pos, rad, gm))) {
                        show_error(text.to_string());
                }
                controller.update_pixbuf();
            }

            drawing_area.queue_draw();
        }),
    );
}

fn setup_mod(
    rbtns: &Vec<gtk::Button>,
    inputs: &Vec<gtk::Entry>,
    controller: &Arc<Mutex<Controller>>,
    drawing_area: &gtk::DrawingArea,
) {
    rbtns[constants::MOD_BTN].connect_clicked(
        clone!(inputs, controller, drawing_area => move |_| {
            let (pos, rad, gm) = match parse_all(&inputs) {
                Ok(res) => res,
                Err(_) => return,
            };
            {
                let mut controller = controller.lock().unwrap();
                if let Err(text) =
                    controller.restruct_muscle(MuscleOperation::Mod(MOParams::new(pos, rad, gm))) {
                        show_error(text.to_string());
                }
                controller.update_pixbuf();
            }

            drawing_area.queue_draw();
        }),
    );
}

fn setup_del(
    rbtns: &Vec<gtk::Button>,
    inputs: &Vec<gtk::Entry>,
    controller: &Arc<Mutex<Controller>>,
    drawing_area: &gtk::DrawingArea,
) {
    rbtns[constants::DEL_BTN].connect_clicked(
        clone!(inputs, controller, drawing_area => move |_| {
            let pos = match parse_or_show_err(inputs[constants::POS_INPUT].get_buffer().get_text()) {
                Ok(val) => val,
                Err(_) => return,
            };

            {
                let mut controller = controller.lock().unwrap();
                if let Err(text) = controller.restruct_muscle(MuscleOperation::Del(pos)) {
                    show_error(text.to_string());
                    return;
                }
                controller.update_pixbuf();
            }

            drawing_area.queue_draw();
        }),
    );
}

fn setup_rpm(
    rbtns: &Vec<gtk::Button>,
    inputs: &Vec<gtk::Entry>,
    controller: &Arc<Mutex<Controller>>,
    drawing_area: &gtk::DrawingArea,
) {
    for (&i1, &i2) in [constants::MODP_BTN, constants::MODPG_BTN]
        .iter()
        .zip([constants::MODM_BTN, constants::MODMG_BTN].iter())
    {
        rbtns[i1].connect_clicked(
        clone!(inputs, controller, drawing_area => move |_| {
            let pos = match parse_or_show_err(inputs[constants::POS_INPUT].get_buffer().get_text()) {
                Ok(val) => val,
                Err(_) => return,
            };
            {
                let mut controller = controller.lock().unwrap();
                let (mut rad, mut gm) = match controller.get_node(pos) {
                    Ok(val) => val,
                    Err(txt) => {
                        show_error(txt);
                        return;
                    },
                };
                match i1 {
                    constants::MODP_BTN => rad += constants::DELTA_VAL,
                    constants::MODPG_BTN => gm += constants::DELTA_VAL,
                    _ => unreachable!("only 2 variants"),
                }
                inputs[constants::RAD_INPUT].get_buffer().set_text(&rad.to_string());
                inputs[constants::GM_INPUT].get_buffer().set_text(&gm.to_string());
                if let Err(text) =
                    controller.restruct_muscle(MuscleOperation::Mod(MOParams::new(pos, rad, gm))) {
                        show_error(text.to_string());
                }
                controller.update_pixbuf();
            }

            drawing_area.queue_draw();
        }),
        );

        rbtns[i2].connect_clicked(
        clone!(inputs, controller, drawing_area => move |_| {
            let pos = match parse_or_show_err(inputs[constants::POS_INPUT].get_buffer().get_text()) {
                Ok(val) => val,
                Err(_) => return,
            };
            {
                let mut controller = controller.lock().unwrap();
                let (mut rad, mut gm) = match controller.get_node(pos) {
                    Ok(val) => val,
                    Err(txt) => {
                        show_error(txt);
                        return;
                    },
                };
                match i2 {
                    constants::MODM_BTN => rad -= constants::DELTA_VAL,
                    constants::MODMG_BTN => gm -= constants::DELTA_VAL,
                    _ => unreachable!("only 2 variants"),
                }
                inputs[constants::RAD_INPUT].get_buffer().set_text(&rad.to_string());
                inputs[constants::GM_INPUT].get_buffer().set_text(&gm.to_string());
                if let Err(text) =
                    controller.restruct_muscle(MuscleOperation::Mod(MOParams::new(pos, rad, gm))) {
                        show_error(text.to_string());
                }
                controller.update_pixbuf();
            }

            drawing_area.queue_draw();
        }),
    );
    }
}

fn setup_next_prev(rbtns: &Vec<gtk::Button>, inputs: &Vec<gtk::Entry>) {
    rbtns[constants::NEXT_BTN].connect_clicked(clone!(inputs => move |_| {
        let mut pos =
            match parse_or_show_err::<usize>(inputs[constants::POS_INPUT].get_buffer().get_text()) {
                Ok(val) => val,
                Err(_) => 0,
            };
        pos += 1;
        inputs[constants::POS_INPUT].get_buffer().set_text(&pos.to_string());
    }));

    rbtns[constants::PREV_BTN].connect_clicked(clone!(inputs => move |_| {
        let mut pos =
            match parse_or_show_err::<usize>(inputs[constants::POS_INPUT].get_buffer().get_text()) {
                Ok(val) => val,
                Err(_) => 0,
            };
        pos -= 1;
        inputs[constants::POS_INPUT].get_buffer().set_text(&pos.to_string());
    }));
}

fn parse_all(inputs: &Vec<gtk::Entry>) -> Result<(usize, f64, f64), ()> {
    let pos = parse_or_show_err(inputs[constants::POS_INPUT].get_buffer().get_text())?;
    let rad = parse_or_show_err(inputs[constants::RAD_INPUT].get_buffer().get_text())?;
    let gm = parse_or_show_err(inputs[constants::GM_INPUT].get_buffer().get_text())?;
    Ok((pos, rad, gm))
}

fn parse_or_show_err<T: std::str::FromStr>(text: String) -> Result<T, ()> {
    match text.parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => {
            show_error(format!("Parse error: {}", text));
            Err(())
        }
    }
}

fn parse_light_pos(inputs: &Vec<gtk::Entry>) -> Result<Point3d, ()> {
    let x = parse_or_show_err(inputs[constants::X_INPUT].get_buffer().get_text())?;
    let y = parse_or_show_err(inputs[constants::Y_INPUT].get_buffer().get_text())?;
    let z = parse_or_show_err(inputs[constants::Z_INPUT].get_buffer().get_text())?;
    Ok(Point3d::new(x, y, z))
}

fn show_error(text: String) {
    let dialog = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::empty(),
        gtk::MessageType::Error,
        gtk::ButtonsType::None,
        &text,
    );
    dialog.set_title("Error");
    dialog.run();
}

fn setup_light_button(
    rbtns: &Vec<gtk::Button>,
    inputs: &Vec<gtk::Entry>,
    controller: &Arc<Mutex<Controller>>,
    drawing_area: &gtk::DrawingArea,
) {
    rbtns[constants::MOVE_LS].connect_clicked(
    clone!(inputs, controller, drawing_area => move |_| {
        let p = match parse_light_pos(&inputs) {
            Ok(val) => val,
            Err(_) => return,
        };
        {
            let mut controller = controller.lock().unwrap();
            controller.move_light_source(p);
            controller.update_pixbuf();
        }

        drawing_area.queue_draw();
    }),
    );
}
