
#[derive(Debug, Fail)]
pub enum ImageErr {
    #[fail(display = "Encountered an invalid pixel count")]
    InvalidPixelCount,
    #[fail(display = "Encountered an invalid char")]
    ParseError,
    #[fail(display = "Something was unexpectedly out of bounds")]
    OutOfBounds,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Pixel {
    Black, // 0
    White, // 1
    Transparent, // 2
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pixels: Vec<Pixel>,
}

impl Layer {
    pub fn new_from_ints(pixels: &Vec<u32>) -> Layer {
        Layer { pixels: pixels.iter().map(|p| match *p {
            0 => Pixel::Black,
            1 => Pixel::White,
            2 => Pixel::Transparent,
            _ => panic!("Invalid pixel"),
        }).collect() }
    }

    pub fn new_from_pixels(pixels: &Vec<Pixel>) -> Layer {
        Layer { pixels: pixels.clone() }
    }

    pub fn pixels(&self) -> &Vec<Pixel> { &self.pixels }

    pub fn pixel(&self, p: usize) -> Option<&Pixel> { self.pixels.get(p) }

    pub fn get_zero_count(&self) -> u32 {
        let mut zero_count = 0;
        for p in &self.pixels {
            if let Pixel::Black = p {
                zero_count += 1;
            }
        }

        zero_count
    }

    pub fn one_cnt_mul_two_cnt(&self) -> u32 {
        let mut one_cnt = 0;
        let mut two_cnt = 0;
        for p in &self.pixels {
            match *p {
                Pixel::White => { one_cnt += 1; },
                Pixel::Transparent => { two_cnt += 1 },
                _ => {}
            }
        }

        one_cnt * two_cnt
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    width: u32,
    height: u32,
    layers: Vec<Layer>,
}

/// ```
/// use aoc_2019::advent_image::{merge_layers, Image, Pixel};
/// let img = Image::new_from_ints(2, 2, &vec![0,2,2,2,1,1,2,2,2,2,1,2,0,0,0,0]).unwrap();
/// let merged = merge_layers(&img).unwrap();
///
/// assert_eq!(merged.layers().get(0).unwrap().pixels()[..], vec![Pixel::Black, Pixel::White, Pixel::White, Pixel::Black][..]);
/// ```
pub fn merge_layers(image: &Image) -> Result<Image, ImageErr> {
    let mut output: Vec<Pixel> = vec![Pixel::Transparent; (image.width() * image.height()) as usize];

    for l in image.layers().iter() {
        for (p_i, p) in l.pixels().iter().enumerate() {
            let existing = output.get(p_i).ok_or(ImageErr::OutOfBounds)?;

            if let Pixel::Transparent = existing {
                std::mem::replace(&mut output[p_i], *p);
            }
        }
    }

    Ok(Image::new_from_layers(
        image.width(),
        image.height(),
        &vec![Layer::new_from_pixels(&output)]
    ))
}

/// ```
/// use aoc_2019::advent_image::{pixels_to_layers, Layer};
/// let layers = pixels_to_layers(3, 2, &vec![0,1,2,0,1,2,0,1,2,0,1,2]).unwrap();
///
/// assert_eq!(layers[..], vec![Layer::new_from_ints(&vec![0,1,2,0,1,2]), Layer::new_from_ints(&vec![0,1,2,0,1,2])][..]);
/// ```
pub fn pixels_to_layers(width: u32, height: u32, pixels: &Vec<u32>) -> Result<Vec<Layer>, ImageErr> {
    let count = (width * height) as usize;
    let mut layers: Vec<Layer> = vec![];

    for (i, _) in pixels.iter().step_by(count).enumerate() {
        if (i + count) > pixels.len() {
            return Err(ImageErr::InvalidPixelCount);
        }

        let pixels_in_layer: Vec<u32> = pixels
            .iter()
            .skip(i * count)
            .take(count)
            .map(|p| *p)
            .collect();

        layers.push(
            Layer::new_from_ints(&pixels_in_layer)
        );
    }

    Ok(layers)
}

impl Image {
    pub fn new_from_ints(width: u32, height: u32, pixels: &Vec<u32>) -> Result<Image, ImageErr> {
        Ok(Image {
            width,
            height,
            layers: pixels_to_layers(width, height, &pixels)?,
        })
    }

    pub fn new_from_layers(width: u32, height: u32, layers: &Vec<Layer>) -> Image {
        Image {
            width,
            height,
            layers: layers.clone(),
        }
    }

    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn day08a_challenge(&self) -> Option<u32> {
        let mut fewest_zeroes = 999;
        let mut challenge: Option<u32> = None;
        for layer in &self.layers {
            let zero_count = layer.get_zero_count();

            if zero_count < fewest_zeroes {
                fewest_zeroes = zero_count;
                challenge = Some(layer.one_cnt_mul_two_cnt());
            }
        }

        challenge
    }

    pub fn day08b_challenge(&self) -> Result<Vec<Vec<Pixel>>, ImageErr> {
        let image = merge_layers(&self)?;

        let layer = image.layers().get(0).ok_or(ImageErr::OutOfBounds)?;

        let mut out: Vec<Vec<Pixel>> = vec![];
        for y in 0..self.height {
            let mut line: Vec<Pixel> = vec![];
            for x in 0..self.width {
                line.push(*layer.pixels().get((x + (y * self.width)) as usize).ok_or(ImageErr::OutOfBounds)?);
            }
            out.push(line);
        }

        Ok(out)
    }
}

