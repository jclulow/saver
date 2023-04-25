use anyhow::Result;
use facade_terminal::image::Dumper;
use facade_terminal::input::InputItem;
use facade_terminal::screen::*;
use termdraw::{Colour, Drawable};

mod palette;
use palette::*;

struct Saver {
    x: f64,
    dx: f64,
    y: f64,
    dy: f64,
    ddy: f64,
}

impl Saver {
    fn new() -> Result<Saver> {
        Ok(Saver { x: 0.0, dx: 1.3, y: 100.0, dy: 0.0, ddy: -0.9 })
    }

    fn draw_oxide(&self, r: &mut dyn Drawable, xbase: isize, ybase: isize) {
        let oxide = include_str!("../templates/oxide.txt")
            .lines()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let oxidew = oxide.iter().map(|l| l.len()).max().unwrap_or(0);

        let mut fox = termdraw::Format::default();
        fox.bg = GREEN_LIGHT;
        fox.fg = Colour::UseExisting;

        let mut pos = 0;

        for y in 0..oxide.len() {
            let yscr = ybase + (y as isize);
            if yscr < 0 {
                continue;
            }

            for x in 0..oxidew {
                if x >= oxide[y].len() {
                    continue;
                }

                let xscr = xbase + (x as isize);
                if xscr < 0 {
                    continue;
                }

                if oxide[y][x] == '#' {
                    r.chrf(
                        xscr.try_into().unwrap(),
                        yscr.try_into().unwrap(),
                        ' ',
                        &fox,
                    );
                }
            }
        }
    }
}

impl Drawer for Saver {
    fn paint(
        &mut self,
        fc: u64,
        r: &mut dyn Drawable,
        de: &mut DrawerExtra,
    ) -> bool {
        let maxx = 100.0; /* XXX */
        let pixwidth = 100.0 / (r.width() as f64);

        let maxy = 100.0;
        let pixheight = 100.0 / (r.height() as f64);

        if self.y >= maxy * 0.85 {
            self.dy = self.dy * 0.85;
        }

        self.y += self.dy;
        self.dy += self.ddy;
        if self.y < pixheight * 7.0 {
            self.y = pixheight * 7.0;
            self.dy = -self.dy;
        }

        self.x += self.dx;
        if self.x > maxx - pixwidth * 39.0 {
            self.x = maxx - pixwidth * 39.0;
            self.dx = -self.dx;
        }
        if self.x < 0.0 {
            self.x = 0.0;
            self.dx = -self.dx;
        }

        let x = (self.x * pixwidth).round() as isize;
        let y = (r.height() as isize)
            - (((self.y / maxy) * (r.height() as f64)).round() as isize);

        self.draw_oxide(r, x, y);
        true
    }

    fn input(&mut self, c: &[u8]) -> bool {
        false
    }
}

fn main() -> Result<()> {
    let mut s = Saver::new()?;

    let dump = Dumper::new(&BLACK);
    drawloop(&mut s, &BLACK, dump, None)
}
