mod wiifit;
mod wiifit_hulahoop;
mod wiifit_sunbullet;
mod wiifit_silhouette;
mod wiifit_silhouettel;

pub fn install() {
    wiifit::install(); 
    wiifit_hulahoop::install();
    wiifit_sunbullet::install();   
    wiifit_silhouette::install();
    wiifit_silhouettel::install();
}