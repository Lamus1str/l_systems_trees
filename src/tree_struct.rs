extern crate rand;

use std::cmp::{min, Ordering};
use std::f32::consts::PI;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::tree_struct::TreeE::{Bud, Segment};

const MAX_MAIN_ANGLE_DELTA: f32 = PI / 6.4;
const BASE_BRANCH_ANGLE: f32 = PI / 5.0;
const MAX_BRANCH_ANGLE_DELTA: f32 = PI / 5.0;
const ANGLE_TO_UP_FORCE: f32 = 0.16;

const GROW_SPEED: f32 = 20.0;
const THIN_UP_SPEED: f32 = 3.6;

const DELTA: f32 = 0.032;
const PERIOD: f32 = 1.6;

fn get_h_coff(h: usize) -> f32{
    1.0 / (h as f32 + 2.0).log2()
}

fn angle_to_up(angle: f32, force: f32) -> f32{
    let new_sin = angle.sin() - force;
    if new_sin < -1.0{
        return -PI / 2.0
    }
    ((1.0 - new_sin.powi(2)).sqrt() * angle.cos().signum()).acos() * new_sin.signum()
}

fn rand_to_coff(rand: f32) -> f32{
    0.5 * (rand * 2.0 - 1.0).powi(3)
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

    fn update(&mut self, delta: f32, state: &UpdaterState){
        match self {
            TreeE::OpBracket => {}
            TreeE::ClBracket => {}
            Segment { length, thickness, angle, lifetime, is_trunk, energy, force } => {
                let mut new_tnick = *thickness + delta * THIN_UP_SPEED * *force * get_h_coff(state.h);
                if let Some(thick_barier) = state.thickness{
                    if thick_barier < new_tnick{
                        new_tnick = *thickness;
                    }
                }

                *self = Segment {
                    length: *length + delta * GROW_SPEED * *force * get_h_coff(state.h),
                    thickness: new_tnick,
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

    fn update_state(
        &mut self, state: &mut UpdaterState,
        stack: &mut Vec<UpdaterState>, brackets: &mut Vec<usize>){
        match self{
            TreeE::OpBracket => {
                brackets.push(state.h);
            }
            TreeE::ClBracket => {
                if let Some(old_h) = brackets.pop(){
                    let mut find = false;
                    while (state.h != old_h) && !stack.is_empty(){
                        if let Some(new_state) = stack.pop(){
                            *state = new_state;
                        }
                    }
                }
            }
            Segment { angle, thickness, .. } => {
                stack.push(state.clone());
                state.angle += *angle;
                state.thickness = Some(*thickness);
                state.h += 1;
                state.parent = self.clone();
            }
            Bud { .. } => {}
            TreeE::Flower => {}
        }
    }

    fn transform(&self, rand_gen: &mut rand::rngs::StdRng, state: &UpdaterState) -> Vec<TreeE> {
        //наверно передовать и направление роста
        let probab = rand_gen.gen_range(0.0..1.0);
        //println!("{probab}");
        let mut result: Vec<TreeE> = Vec::new();
        match self{
            TreeE::Bud { energy, .. } => {
                if *energy >= PERIOD{
                    match probab {
                        0.0..=0.05 => {
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
                        0.3..=0.301 => {
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
                if *energy >= PERIOD{
                    match probab {
                        0.0..=0.1 => {
                            result.push(TreeE::Segment {
                                angle: *angle,
                                length: *length,
                                thickness: *thickness,
                                lifetime: *lifetime,
                                is_trunk: true,
                                energy: 0.0,
                                force: force * 0.2,
                            });
                            let new_base_angle = angle_to_up(state.angle, ANGLE_TO_UP_FORCE) - state.angle;
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
        result
    }
}

pub struct Tree{
    pub l_system: Vec<TreeE>,
    rand_gen: rand::rngs::StdRng,
}

#[derive(Clone)]
struct UpdaterState{
    angle: f32,
    h: usize,
    parent: TreeE,
    thickness: Option<f32>,
}
impl Tree {
    pub fn new(seed: [u8; 32]) -> Tree{
        Tree{
            l_system: vec![TreeE::new_segment(1.0, 0.0)],
            rand_gen: StdRng::from_seed(seed),
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
        let mut new_l: Vec<TreeE> = Vec::new();

        let mut state: UpdaterState = UpdaterState{
            angle: -0.5 * PI,
            h: 0,
            parent: TreeE::OpBracket,
            thickness: None,
        };
        let mut stack: Vec<UpdaterState> = Vec::new();
        let mut brackets_stack: Vec<usize> = Vec::new();

        for ch in &mut self.l_system {
            ch.update(DELTA, &state);
            new_l.append(&mut ch.transform(&mut self.rand_gen, &state));
            ch.update_state(&mut state, &mut stack, &mut brackets_stack);
        }
        self.l_system = new_l;
    }
}