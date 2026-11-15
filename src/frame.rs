pub struct Frame{
    name: String,
    lexical_units: Vec<String>,
    frame_elements: Vec<String>
}

impl Frame{
    /// Constructor of {@link Frame} class which takes frame name as input
    ///
    /// # Arguments
    ///
    /// * `name`: Name of the frame
    ///
    /// returns: Frame
    ///
    pub fn new(name: String) -> Frame{
        Self{name, lexical_units: Vec::new(), frame_elements: Vec::new()}
    }

    /// Adds a new lexical unit to the current frame
    ///
    /// # Arguments
    ///
    /// * `lexical_unit`: Lexical unit to be added
    ///
    pub fn add_lexical_unit(&mut self, lexical_unit: String){
        self.lexical_units.push(lexical_unit);
    }

    /// Adds a new frame element to the current frame
    ///
    /// # Arguments
    ///
    /// * `frame_element`: Frame element to be added
    ///
    pub fn add_frame_element(&mut self, frame_element: String){
        self.frame_elements.push(frame_element);
    }

    /// Checks if the given lexical unit exists in the current frame
    ///
    /// # Arguments
    ///
    /// * `lexical_unit`: Lexical unit to be searched.
    ///
    /// returns: bool True if the lexical unit exists, false otherwise.
    ///
    pub fn lexical_unit_exists(&self, lexical_unit: &str) -> bool{
        self.lexical_units.iter().any(|lu| lu == lexical_unit)
    }

    /// Accessor for a given index in the lexicalUnit array.
    ///
    /// # Arguments
    ///
    /// * `index`: Index of the lexical unit
    ///
    /// returns: &str The lexical unit at position index in the lexicalUnit array
    ///
    pub fn get_lexical_unit(&self, index: usize) -> &str{
        &self.lexical_units[index]
    }

    /// Accessor for a given index in the frameElements array.
    ///
    /// # Arguments
    ///
    /// * `index`: Index of the frame element
    ///
    /// returns: &str The frame element at position index in the frameElements array
    ///
    pub fn get_frame_element(&self, index: usize) -> &str{
        &self.frame_elements[index]
    }

    /// Returns number of lexical units in the current frame
    ///
    /// returns: usize Number of lexical units in the current frame
    ///
    pub fn lexical_unit_size(&self) -> usize{
        self.lexical_units.len()
    }

    /// Returns number of frame elements in the current frame
    ///
    /// returns: usize Number of frame elements in the current frame
    ///
    pub fn frame_elements_size(&self) -> usize{
        self.frame_elements.len()
    }

    /// Accessor for the name of the frame
    ///
    /// returns: &str Name of the frame
    ///
    pub fn get_name(&self) -> &str{
        &self.name
    }
}