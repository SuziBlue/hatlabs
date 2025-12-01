use crate::{borders::BorderType, components::TextStyle};

pub struct Style {
    pub border: BorderType,
    pub thick_border: BorderType,

    pub h1_text: TextStyle,
    pub h2_text: TextStyle,
    pub h3_text: TextStyle,
    pub h4_text: TextStyle,
    pub h5_text: TextStyle,
    pub h6_text: TextStyle,
    pub body_text: TextStyle,
    pub subsection_text: TextStyle,
    
}

