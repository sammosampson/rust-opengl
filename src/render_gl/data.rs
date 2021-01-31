use gl;
extern crate vec_2_10_10_10;

pub struct AttributePointer {
    attribute_type: gl::types::GLuint,
    normalised: bool,
    stride: gl::types::GLint,
    location: gl::types::GLuint,
    offset: *const gl::types::GLvoid,
    component_count: gl::types::GLint,
}

impl AttributePointer { 
    fn for_float() -> Self{
        Self {
            attribute_type: gl::FLOAT,
            normalised: false,
            stride: Default::default(),
            location: Default::default(),
            offset: 0 as *const gl::types::GLvoid,
            component_count: Default::default(),
        }
    }

    fn for_uint_2_10_10_10_rev() -> Self{
        Self {
            attribute_type: gl::UNSIGNED_INT_2_10_10_10_REV,
            normalised: false,
            stride: Default::default(),
            location: Default::default(),
            offset: 0 as *const gl::types::GLvoid,
            component_count: Default::default(),
        }
    }

    fn for_byte() -> Self{
        Self {
            attribute_type: gl::BYTE,
            normalised: false,
            stride: Default::default(),
            location: Default::default(),
            offset: 0 as *const gl::types::GLvoid,
            component_count: Default::default(),
        }
    }

    fn for_unsigned_byte() -> Self{
        Self {
            attribute_type: gl::UNSIGNED_BYTE,
            normalised: false,
            stride: Default::default(),
            location: Default::default(),
            offset: 0 as *const gl::types::GLvoid,
            component_count: Default::default(),
        }
    }

    fn for_unsigned_short() -> Self{
        Self {
            attribute_type: gl::UNSIGNED_SHORT,
            normalised: false,
            stride: Default::default(),
            location: Default::default(),
            offset: 0 as *const gl::types::GLvoid,
            component_count: Default::default(),
        }
    }
    
    fn is_normalised(&self) -> Self{
        Self {
            attribute_type: self.attribute_type,
            normalised: true,
            stride: self.stride,
            location: self.location,
            offset: self.offset,
            component_count: self.component_count
        }
    }
    
    fn with_offset(&self, offset: usize) -> Self{
        Self {
            attribute_type: self.attribute_type,
            normalised: self.normalised,
            stride: self.stride,
            location: self.location,
            offset: offset as *const gl::types::GLvoid,
            component_count: self.component_count
        }
    }

    fn with_stride(&self, stride: usize) -> Self {
        Self {
            attribute_type: self.attribute_type,
            normalised: self.normalised,
            stride: stride as gl::types::GLint,
            location: self.location,
            offset: self.offset,
            component_count: self.component_count
        }
    }

    fn with_location(&self, location: usize) -> Self {
        Self {
            attribute_type: self.attribute_type,
            normalised: self.normalised,
            stride: self.stride,
            location: location as gl::types::GLuint,
            offset: self.offset,
            component_count: self.component_count,
        }
    }

    fn with_component_count(&self, component_count: usize) -> Self {
        Self {
            attribute_type: self.attribute_type,
            normalised: self.normalised,
            stride: self.stride,
            location: self.location,
            offset: self.offset,
            component_count: component_count as gl::types::GLint,
        }
    }

    fn set_used(&self, gl: &gl::Gl) {
        unsafe {
            gl.EnableVertexAttribArray(self.location);
            if self.attribute_type == gl::UNSIGNED_SHORT {
                gl.VertexAttribIPointer(
                    self.location,
                    self.component_count,
                    self.attribute_type,
                    self.stride,
                    self.offset
                );
            } else {
                gl.VertexAttribPointer(
                    self.location,
                    self.component_count,
                    self.attribute_type,
                    if self.normalised { gl::TRUE } else { gl::FALSE },
                    self.stride,
                    self.offset
                );
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,
}

impl f32_f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32, d3: f32) -> f32_f32_f32_f32 {
        f32_f32_f32_f32 {
            d0, d1, d2, d3
        }
    }
    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_float()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(4)
            .set_used(gl);
    }
}

impl From<(f32, f32, f32, f32)> for f32_f32_f32_f32 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        f32_f32_f32_f32::new(other.0, other.1, other.2, other.3)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32 {
            d0, d1, d2
        }
    }
    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_float()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(3)
            .set_used(gl);
    }
}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32 {
    pub d0: f32,
    pub d1: f32,
}

impl f32_f32 {
    pub fn new(d0: f32, d1: f32) -> f32_f32 {
        f32_f32 {
            d0, d1
        }
    }
    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_float()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(2)
            .set_used(gl);
    }
}

impl From<(f32, f32)> for f32_f32{
    fn from(other: (f32, f32)) -> Self {
        f32_f32::new(other.0, other.1)
    }
}

impl From<(u16, u16)> for f32_f32{
    fn from(other: (u16, u16)) -> Self {
        f32_f32::new(other.0 as f32, other.1 as f32)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_ {
    pub d0: f32
}

impl f32_ {
    pub fn new(d0: f32) -> f32_ {
        f32_ {
            d0
        }
    }
    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_float()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(1)
            .set_used(gl);
    }
}

impl From<f32> for f32_{
    fn from(other: f32) -> Self {
        f32_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
    pub inner: ::vec_2_10_10_10::Vector,
}

impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev_float {
            inner: ::vec_2_10_10_10::Vector::new(other.0, other.1, other.2, other.3)
        }
    }
}

impl u2_u10_u10_u10_rev_float {
    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_uint_2_10_10_10_rev()
            .is_normalised()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(4)
            .set_used(gl);
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u8_ {
    pub d0: u8,
}

impl u8_ {
    pub fn new(d0: u8) -> u8_ {
        u8_ { d0 }
    }

    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_unsigned_byte()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(1)
            .set_used(gl);
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16_u16_u16 {
    pub d0: u16,
    pub d1: u16,
    pub d2: u16,
    pub d3: u16,
}

impl u16_u16_u16_u16 {
    pub fn new(d0: u16, d1: u16, d2: u16, d3: u16) -> u16_u16_u16_u16 {
        u16_u16_u16_u16 {
            d0, d1, d2, d3
        }
    }
    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_unsigned_short()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(4)
            .set_used(gl);
    }
}

impl From<(u16, u16, u16, u16)> for u16_u16_u16_u16 {
    fn from(other: (u16, u16, u16, u16)) -> Self {
        u16_u16_u16_u16::new(other.0, other.1, other.2, other.3)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u16_u16 {
    pub d0: u16,
    pub d1: u16
}

impl u16_u16 {
    pub fn new(d0: u16, d1: u16) -> u16_u16 {
        u16_u16 {
            d0, d1
        }
    }
    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_unsigned_short()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(2)
            .set_used(gl);
    }
}

impl From<(u16, u16)> for u16_u16 {
    fn from(other: (u16, u16)) -> Self {
        u16_u16::new(other.0, other.1)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u32_u32_u32_u32 {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
    pub d3: u32,
}

impl u32_u32_u32_u32 {
    pub fn new(d0: u32, d1: u32, d2: u32, d3: u32) -> u32_u32_u32_u32 {
        u32_u32_u32_u32 {
            d0, d1, d2, d3
        }
    }
    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_unsigned_short()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(4)
            .set_used(gl);
    }
}

impl From<(u32, u32, u32, u32)> for u32_u32_u32_u32 {
    fn from(other: (u32, u32, u32, u32)) -> Self {
        u32_u32_u32_u32::new(other.0, other.1, other.2, other.3)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_ {
    pub d0: i8,
}

impl i8_ {
    pub fn new(d0: i8) -> i8_ {
        i8_ { d0 }
    }

    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_byte()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(1)
            .set_used(gl);
    }
}

impl From<u8> for u8_ {
    fn from(other: u8) -> Self {
        u8_::new(other)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct i8_float {
    pub d0: i8,
}

impl i8_float {
    pub fn new(d0: i8) -> i8_float {
        i8_float { d0 }
    }

    pub fn set_gl_attribute_pointer(gl: &gl::Gl, stride: usize, location: usize, offset: usize) {
        AttributePointer::for_byte()
            .is_normalised()
            .with_offset(offset)
            .with_location(location)
            .with_stride(stride)
            .with_component_count(1)
            .set_used(gl);
    }
}

impl From<i8> for i8_float { 
    fn from(other: i8) -> Self {
        i8_float::new(other)
    }
}