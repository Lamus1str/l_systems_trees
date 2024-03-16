extern crate rand;

use std::f32::consts::PI;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::tree_struct::TreeE::{Bud, Segment};

const MAX_MAIN_ANGLE_DELTA: f32 = PI / 6.0;
const BASE_BRANCH_ANGLE: f32 = PI / 4.0;
const MAX_BRANCH_ANGLE_DELTA: f32 = PI / 6.0;
const ANGLE_TO_UP_FORCE: f32 = 0.06;

const GROW_SPEED: f32 = 6.0;
const THIN_UP_SPEED: f32 = 1.5;


fn angle_to_up(angle: f32, force: f32) -> f32{
    let new_sin = angle.sin() + force;
    if new_sin > 1.0{
        return PI / 2.0
    }
    ((1.0 - new_sin.powi(2)).sqrt() * angle.cos().signum()).acos() * new_sin.signum()
}

fn rand_to_coff(rand: f32) -> f32{
    0.5 * (rand * 2.0 - 1.0).powi(3)
}

pub fn test() -> Tree{
    let mut ang = 0.0;
    let mut ang2 = PI;
    for i in 0..5{
        //println!("{ang} {ang2}");
        ang = angle_to_up(ang, 0.08);
        ang2 = angle_to_up(ang2, 0.08);
    }

    let mut tr = Tree::new();
    for i in 0..128{
        tr.update();
    }
    tr.printthis();
    return tr;
}


#[derive(Clone, Debug)]
pub enum TreeE{
    OpBracket,
    ClBracket,
    Segment{
        length: f32,
        thickness: f32,
        angle: f32,
        lifetime: f32,
        is_trunk: bool,
        energy: f32,
        force: f32 },
    Bud{ angle: f32, energy: f32, is_main: bool, sheet_angle: f32, sheet_size: f32 },
    Flower,
}

impl TreeE{
    fn new_char(ch: char) -> TreeE{
        TreeE::OpBracket
    }
    fn segment_force_from_noise(noise: f32) -> f32{
        0.0
    }
    fn new_segment_from_bud(bud: TreeE, force: f32) -> TreeE{
        if let TreeE::Bud {
            angle, ..
        } = bud{
            return Segment {
                angle,
                length: 0.0,
                thickness: 0.0,
                lifetime: 0.0,
                is_trunk: false,
                energy: 0.0,
                force,
            }
        }
        TreeE::OpBracket
    }
    fn new_segment(force: f32, angle: f32) -> TreeE{
        Segment {
            angle: angle,
            length: 0.0,
            thickness: 0.0,
            lifetime: 0.0,
            is_trunk: false,
            energy: 0.0,
            force: force,
        }
    }
    fn new_bud(is_main: bool, angle: f32, sheet_angle: f32) -> TreeE{
        TreeE::Bud {
            angle,
            is_main,
            energy: 0.0,
            sheet_angle,
            sheet_size: 0.0
        }
    }
    fn new_trunk() -> TreeE{
        TreeE::OpBracket
    }
    fn from_char(ch: char, from: TreeE) -> TreeE{
        match ch {
            't' => {
                //Branch
            }
            'T' => {
                //Trunk
            }
            'B' => {
                //Bud
            }
            _ => {

            }
        }
        TreeE::OpBracket
    }
    fn get_char(&self) -> char{
        'F'
    }

    fn update(&mut self, delta: f32){
        match self {
            TreeE::OpBracket => {}
            TreeE::ClBracket => {}
            Segment { length, thickness, angle, lifetime, is_trunk, energy, force } => {
                *self = Segment {
                    length: *length + delta * GROW_SPEED * *force,
                    thickness: *thickness + delta * THIN_UP_SPEED * *force,
                    angle: *angle,
                    lifetime: *lifetime + delta,
                    is_trunk: *is_trunk,
                    energy: *energy + delta,
                    force: *force
                };
            }
            Bud { angle, energy, is_main, sheet_angle, sheet_size } => {
                *self = TreeE::Bud {
                    angle: *angle,
                    energy: *energy + delta,
                    is_main: *is_main,
                    sheet_angle: *sheet_angle,
                    sheet_size: *sheet_size
                }
            }
            TreeE::Flower => {}
        }
    }
    fn transform(&self, rand_gen: &mut rand::rngs::StdRng) -> Vec<TreeE> {
        //наверно передовать и направление роста
        let probab = rand_gen.gen_range(0.0..1.0);
        //println!("{probab}");
        let mut result: Vec<TreeE> = Vec::new();
        match self{
            TreeE::Bud { energy, .. } => {
                if *energy >= 1.0{
                    match probab {
                        0.0..=0.6 => {
                            result.push(TreeE::new_segment_from_bud(
                                self.clone(),
                                1.0 + rand_to_coff(rand_gen.gen_range(0.0..1.0))));
                            result.push(TreeE::OpBracket);
                            //result.push(TreeE::new_bud(false, 0.0,0.0));
                            //Им ей сразу же(почке) задаём напрпавление
                            result.push(TreeE::ClBracket);

                            result.push(TreeE::OpBracket);
                            //result.push(TreeE::new_segment(1.0, 0.0));
                            result.push(TreeE::ClBracket);
                            //Второе, что создаётся сразу же, это основная ветвь
                            //to branch
                        },
                        0.4..=0.6 => {
                            //to flower
                        },
                        0.6..=0.7 => {
                            //death
                        }
                        _ => {
                            result.push(self.clone())
                        }
                    }
                }
                else{
                    result.push(self.clone())
                }
            }
            TreeE::Segment {
                angle,
                length,
                thickness,
                lifetime,
                is_trunk: false,
                energy,
                force,
            } => {
                if *energy >= 1.0{
                    match probab {
                        0.0..=0.4 => {

                            //println!("new_seg");
                            result.push(TreeE::Segment {
                                angle: *angle,
                                length: *length,
                                thickness: *thickness,
                                lifetime: *lifetime,
                                is_trunk: true,
                                energy: 0.0,
                                force: force * 0.5,
                            });
                            //result.push(TreeE::new_segment(0.0));
                            //Следующий сегмент, по направлению он должен стремится к тому,
                            // чтобы расти вверх, но не слишком резко и с рандомными отклонениями

                            let new_base_angle = angle_to_up(*angle, ANGLE_TO_UP_FORCE);
                            let main_angle = new_base_angle + MAX_MAIN_ANGLE_DELTA *
                                rand_to_coff(rand_gen.gen_range(0.0..1.0)) * 2.0;
                            let branch_angle = new_base_angle + BASE_BRANCH_ANGLE * -main_angle.signum() +
                                MAX_BRANCH_ANGLE_DELTA * rand_to_coff(rand_gen.gen_range(0.0..1.0)) * 2.0;

                            result.push(TreeE::OpBracket);
                            result.push(TreeE::new_bud(false, branch_angle,0.0));
                            //Им ей сразу же(почке) задаём напрпавление
                            result.push(TreeE::ClBracket);

                            result.push(TreeE::OpBracket);
                            result.push(TreeE::new_segment(
                                1.0 + rand_to_coff(rand_gen.gen_range(0.0..1.0)),
                                main_angle
                            ));
                            result.push(TreeE::ClBracket);
                        }
                        _ => {
                            result.push(self.clone())
                        }
                    }
                }else {
                    result.push(self.clone())
                }
            }
            _ => {
                result.push(self.clone())
            }
        }
        //println!("{:?}", result);
        result

    }
}

pub struct Tree{
    pub l_system: Vec<TreeE>,
    rand_gen: rand::rngs::StdRng,
}

struct StackE{

}
impl Tree {
    fn new() -> Tree{
        Tree{
            l_system: vec![TreeE::new_segment(1.0, 0.0)],
            rand_gen: StdRng::from_seed([128; 32]),
        }
    }
    fn printthis(&self){
        for i in &self.l_system{
            match i {
                TreeE::Bud { .. } => print!("B"),
                TreeE::Segment { .. } => print!("S"),
                TreeE::OpBracket => print!("("),
                TreeE::ClBracket => print!(")"),
                TreeE::Flower => print!("F"),
            }
        }
        print!("\n");
    }

    pub fn update(&mut self){
        let mut stack = 0.0;
        let mut new_l: Vec<TreeE> = Vec::new();
        for ch in &mut self.l_system {
            ch.update(0.02);
            new_l.append(&mut ch.transform(&mut self.rand_gen));
        }
        self.l_system = new_l;
    }
}