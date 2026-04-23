use std::{io::{self, Write}, iter::Map};
use clearscreen::clear;
use std::thread;
use std::time::Duration;
use rand::{Rng, RngExt}; //don't change ts plz

macro_rules! placeholder {
    () => {
        println!("hello world!")
    };
}

static mut MAP: [[char; 3]; 3] = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
static mut PLRPOINTS: [i32; 6] = [0, 0, 0, 0, 0, 0]; //lignes 123 puis colones ABC
static mut PLRWIN: bool = false;

fn drawmap()
{
    println!("    A   B   C");
    let mut nbline: i32 = 0;
    unsafe {
        for line in &*(&raw const MAP) {
            nbline += 1;
            println!("{} | {} | {} | {} |", nbline, line[0], line[1], line[2]);
        }
    }
}

// Retourne 'true' si le coup a été joué, 'false' si l'entrée était invalide
fn chng_map(coords: (char, char), w_plr: char) -> bool {
    unsafe {
        match (coords.0, coords.1) {
            ('A', '1') => { if MAP[0][0] == ' ' {if w_plr == 'X' {PLRPOINTS[3] += 1; PLRPOINTS[0] += 1;} MAP[0][0] = w_plr; true } else {println!("Coordonnées occupées\n"); thread::sleep(Duration::from_millis(750)); false} }
            ('A', '2') => { if MAP[1][0] == ' ' {if w_plr == 'X' {PLRPOINTS[3] += 1; PLRPOINTS[1] += 1;} MAP[1][0] = w_plr; true } else {println!("Coordonnées occupées\n"); thread::sleep(Duration::from_millis(750)); false} }
            ('A', '3') => { if MAP[2][0] == ' ' {if w_plr == 'X' {PLRPOINTS[3] += 1; PLRPOINTS[2] += 1;} MAP[2][0] = w_plr; true } else {println!("Coordonnées occupées\n"); thread::sleep(Duration::from_millis(750)); false} }
            
            ('B', '1') => { if MAP[0][1] == ' ' {if w_plr == 'X' {PLRPOINTS[4] += 1; PLRPOINTS[0] += 1;} MAP[0][1] = w_plr; true } else {println!("Coordonnées occupées\n"); thread::sleep(Duration::from_millis(750)); false} }
            ('B', '2') => { if MAP[1][1] == ' ' {if w_plr == 'X' {PLRPOINTS[4] += 1; PLRPOINTS[1] += 1;} MAP[1][1] = w_plr; true } else {println!("Coordonnées occupées\n"); thread::sleep(Duration::from_millis(750)); false} }
            ('B', '3') => { if MAP[2][1] == ' ' {if w_plr == 'X' {PLRPOINTS[4] += 1; PLRPOINTS[2] += 1;} MAP[2][1] = w_plr; true } else {println!("Coordonnées occupées\n"); thread::sleep(Duration::from_millis(750)); false} }
            
            ('C', '1') => { if MAP[0][2] == ' ' {if w_plr == 'X' {PLRPOINTS[5] += 1; PLRPOINTS[0] += 1;} MAP[0][2] = w_plr; true } else {println!("Coordonnées occupées\n"); thread::sleep(Duration::from_millis(750)); false} }
            ('C', '2') => { if MAP[1][2] == ' ' {if w_plr == 'X' {PLRPOINTS[5] += 1; PLRPOINTS[1] += 1;} MAP[1][2] = w_plr; true } else {println!("Coordonnées occupées\n"); thread::sleep(Duration::from_millis(750)); false} }
            ('C', '3') => { if MAP[2][2] == ' ' {if w_plr == 'X' {PLRPOINTS[5] += 1; PLRPOINTS[2] += 1;} MAP[2][2] = w_plr; true } else {println!("Coordonnées occupées\n"); thread::sleep(Duration::from_millis(750)); false} }
            
            _ => {
                println!("Entrée incorrecte");
                thread::sleep(Duration::from_millis(750));
                false
            }
        }
    }
}

fn checkwin()
{
    unsafe {
        for points in PLRPOINTS {
            if points > 2 || MAP[0][0] == 'X' && MAP[1][1] == 'X' && MAP[2][2] == 'X' || MAP[0][2] == 'X' && MAP[1][1] == 'X' && MAP[2][0] == 'X'{
                PLRWIN = true;
                break;
            } else {PLRWIN = false;}
        }
    }
}

fn plr_play() { // On définit un type de retour (tuple)
    loop {
        println!("Veuillez entrer les coordonnées (ex: A1) :");
        io::stdout().flush().unwrap();

        let mut plr_resp = String::new();
        io::stdin()
            .read_line(&mut plr_resp)
            .expect("erreur de la lecture");

        let mut chars = plr_resp.trim().chars();
        
        let plr_x = chars.next().unwrap_or(' '); 
        let plr_y = chars.next().unwrap_or(' '); 

        if chng_map((plr_x.to_ascii_uppercase(), plr_y), 'X') {
            break;
        }
        clearscreen::clear().expect("echec du clearscreen");
        drawmap();
    }
}

fn ai_check_around(_simple_vec_map: [char; 9], i: i32) {
    match i {
        0 => {
            if _simple_vec_map[1] == ' '
            { chng_map(('B', '1'), 'O'); } else if _simple_vec_map[3] == ' '
            { chng_map(('C', '1'), 'O'); } else if _simple_vec_map[4] == ' '
            { chng_map(('B', '2'), 'O'); }
        }
        1 => {
            if _simple_vec_map[0] == ' '
            { chng_map(('A', '1'), 'O'); } else if _simple_vec_map[2] == ' '
            { chng_map(('A', '3'), 'O'); } else if _simple_vec_map[4] == ' '
            {chng_map(('B', '2'), 'O'); }
        }
        2 => {
            if _simple_vec_map[1] == ' '
            { chng_map(('B', '1'), 'O'); } else if _simple_vec_map[4] == ' '
            { chng_map(('B', '2'), 'O'); } else if _simple_vec_map[5] == ' '
            { chng_map(('C', '2'), 'O'); }
        }
        3 => {
            if _simple_vec_map[0] == ' '
            { chng_map(('A', '1'), 'O'); } else if _simple_vec_map[4] == ' '
            { chng_map(('B', '2'), 'O'); } else if _simple_vec_map[6] == ' '
            { chng_map(('A', '3'), 'O'); }
        }
        4 => {
            if _simple_vec_map[1] == ' '
            { chng_map(('B', '1'), 'O'); } else if _simple_vec_map[3] == ' '
            { chng_map(('A', '2'), 'O'); } else if _simple_vec_map[5] == ' '
            { chng_map(('C', '2'), 'O'); } else if _simple_vec_map[7] == ' '
            { chng_map(('B', '3'), 'O'); }
        }
        5 => {
            if _simple_vec_map[2] == ' '
            { chng_map(('A', '3'), 'O'); } else if _simple_vec_map[4] == ' '
            { chng_map(('B', '2'), 'O'); } else if _simple_vec_map[8] == ' '
            { chng_map(('C', '3'), 'O'); }
        }
        6 => {
            if _simple_vec_map[3] == ' '
            { chng_map(('A', '2'), 'O'); } else if _simple_vec_map[4] == ' '
            { chng_map(('B', '2'), 'O'); } else if _simple_vec_map[7] == ' '
            { chng_map(('B', '3'), 'O'); }
        }
        7 => {
            if _simple_vec_map[4] == ' '
            { chng_map(('B', '2'), 'O'); } else if _simple_vec_map[6] == ' '
            { chng_map(('A', '3'), 'O'); } else if _simple_vec_map[8] == ' '
            { chng_map(('C', '3'), 'O'); }
        }
        8 => {
            if _simple_vec_map[5] == ' '
            { chng_map(('C', '2'), 'O'); } else if _simple_vec_map[7] == ' '
            { chng_map(('B', '3'), 'O'); } else if _simple_vec_map[4] == ' '
            { chng_map(('B', '2'), 'O'); }
        }
        _ => {
            println!("y'a un blème au niveau de i (ia avancée)")
        }
    }
}

fn adv_ai_play(rng: &mut impl Rng) {
    let mut _simple_vec_map: [char; 9] = [' '; 9];
    unsafe {
        let mut i = 0;
        for line in MAP {
            for &c in &line {
                _simple_vec_map[i] = c;
                i += 1;
            }
        }
    }
    let mut already_played: bool = false;
    for pos in &_simple_vec_map {
        if *pos != 'O' {
            already_played = false;
        }
    }
    if already_played {
        let mut i: i32 = 0;
        for pos in &_simple_vec_map {
            ai_check_around(_simple_vec_map, i);
            i += 1;
        }
    } else {rnd_ai_play(rng);}

}

fn rnd_ai_play(rng: &mut impl Rng) {
    loop {
        // C'est ICI que la magie opère en v0.10+
        let x_idx = rng.random_range(0..3); 
        let y_idx = rng.random_range(0..3);
        
        let x_char = match x_idx { 0 => 'A', 1 => 'B', _ => 'C' };
        let y_char = match y_idx { 0 => '1', 1 => '2', _ => '3' };
        
        if chng_map((x_char, y_char), 'O') {
            break;
        }
    }
}

fn main()
{
    let mut rng = rand::rng();
    println!("Commencement du jeu...");
    thread::sleep(Duration::from_millis(500));
        unsafe {
        while !PLRWIN {
            clearscreen::clear().expect("echec du clearscreen");
            drawmap();
            plr_play();
            checkwin();
            if PLRWIN {
                break;
            }
            adv_ai_play(&mut rng);
            checkwin();
        }
        clearscreen::clear().expect("echec du clearscreen");
        drawmap();
        println!("Vous avez gagné.");
        thread::sleep(Duration::from_secs(1));
        clearscreen::clear().expect("echec du clearscreen");
    }
}