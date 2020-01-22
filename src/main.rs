extern crate csv_import_general;
extern crate gnuplot;

use csv_import_general::csv_parse;


use std::error::Error;
use std::process;

use gnuplot::*;

#[derive(Debug)]
struct PROBABILITY{
    pub element :i32,
    pub probability :f64
}

#[derive(Debug)]
struct AXIS_RANGE{
    pub x_max :f64,
    pub x_min :f64,
    pub y_max :f64,
    pub y_min :f64,
}

fn make_probability(datas :&Vec<i32>,max_size : usize) -> Result< Vec<PROBABILITY> , () >{
    let mut probs : Vec<PROBABILITY> = Vec::new();
    let mut prob_element_check_flag : bool = false;
    let mut p_buff : PROBABILITY;

    for item in datas{
        for mut p_item in &mut probs{
            if p_item.element == *item {
                p_item.probability = p_item.probability + 1.0;
                prob_element_check_flag = true;
            }
        }
        if prob_element_check_flag == false {
            let p_buff  = PROBABILITY {element: *item , probability: 1.0};
            probs.push(p_buff);
        }
        prob_element_check_flag = false;
    }
    probs.sort_by(|a, b| a.element.cmp(&b.element));
    for item in &mut probs{
        item.probability = item.probability / ( max_size as f64);
    }

    Ok(probs)
}

fn plot(datas : &Vec<PROBABILITY>,fg : &mut Figure , color : String,caption : String, axis_info : &AXIS_RANGE){
    let mut x : Vec<i32> = Vec::new();
    let mut y : Vec<f64> = Vec::new();
    for item in datas {
        x.push(item.element);
        y.push(item.probability);
    }

    fg.axes2d()
        .set_x_range(Fix(axis_info.x_min), Fix(axis_info.x_max))
        .set_y_range(Fix(axis_info.y_min),Fix(axis_info.y_max))
        .lines_points(&x, &y, &[Color(&color)])
        .set_x_axis(true, &[]);

}

fn ave_calc(datas :&Vec<i32>) -> f64{
    let mut sum = 0.0;
    for item in datas{
        sum += *item as f64;
    }

    sum/(datas.len() as f64)
}

fn normal_dist_calc(data :f64,ave : f64, sigma :f64) -> f64{
    let tempconstnum = 1.0/((2.0*std::f64::consts::PI).sqrt()*sigma.sqrt());
    let tempexp = -((data-ave).powf(2.0)/(2.0*sigma));
    tempconstnum*tempexp.exp()

}

fn main() {
    let mut datas_raw = csv_parse::read_csv_data("plane_data-2020-01-21.csv".to_string()).unwrap();
    let mut datas : Vec<i32> = Vec::new();
    for item in &datas_raw{
        // println!("{:?}",item.get(0).unwrap().parse::<i32>().unwrap());
        datas.push(item.get(0).unwrap().parse::<i32>().unwrap());
    }

    let ave = ave_calc(&datas);
    println!("{}",ave);
    let ave_for_plot_x = vec![ave,ave];
    let ave_for_plot_y = vec![0.0,1.0];

    //S
    let mut S = 0.0;
    for item in &datas{
        S += (*item as f64 - ave).powf(2.0);
    }
    println!("S {} , len {}",S, datas_raw.len() );
    let sigma = S/(datas_raw.len() as f64);

    let mut normal_dist_x : Vec<i32> = Vec::new();
    let mut normal_dist_y : Vec<f64> = Vec::new();
    
    for num in 0..4000{
        normal_dist_y.push(1.0 - (normal_dist_calc(num as f64,ave,sigma)/normal_dist_calc(ave,ave,sigma)));
        normal_dist_x.push(num);
    }

    let mut plot_data = Figure::new();
    let range = &AXIS_RANGE { x_min : 1000.0 ,x_max : 4000.0 ,y_min : 0.0 ,y_max : 1.0};
    plot(&make_probability(&datas,datas.len()).unwrap(),&mut plot_data,"blue".to_string(),"probs".to_string(),range);

    //average line
    plot_data.axes2d()
        .set_x_range(Fix(range.x_min), Fix(range.x_max))
        .set_y_range(Fix(range.y_min),Fix(range.y_max))
        .lines_points(&ave_for_plot_x, &ave_for_plot_y, &[Color("red")])
        .set_x_axis(true, &[]);

    //normal_dist line
    plot_data.axes2d()
        .set_x_range(Fix(range.x_min), Fix(range.x_max))
        .set_y_range(Fix(range.y_min),Fix(range.y_max))
        .lines_points(&normal_dist_x, &normal_dist_y, &[Color("green")])
        .set_x_axis(true, &[]);

    plot_data.set_title("probabilities");
    plot_data.show();
}
