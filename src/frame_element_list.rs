use crate::frame_element::FrameElement;

pub struct FrameElementList {
    frame_elements: Vec<FrameElement>
}

impl FrameElementList {

    /// Constructor of frame element list from a string. The frame elements for a word is a concatenated list of
    //  frame element separated via '#' character.
    ///
    /// # Arguments
    ///
    /// * `frame_element_list`: String consisting of frame elements separated with '#' character.
    ///
    /// returns: FrameElementList
    ///
    pub fn new(frame_element_list: String) -> FrameElementList {
        let mut frame_elements = Vec::new();
        let items = frame_element_list.split("#");
        for item in items {
            frame_elements.push(FrameElement::from_string(item.to_string()));
        }
        Self{frame_elements}
    }

    /// Replaces id's of predicates, which have previousId as synset id, with currentId.
    ///
    /// # Arguments
    ///
    /// * `previous_id`: Previous id of the synset.
    /// * `current_id`: Replacement id.
    ///
    pub fn update_connected_id(&mut self, previous_id: String, current_id: String){
        for frame_element in self.frame_elements.iter_mut() {
            if frame_element.get_id() == previous_id {
                frame_element.set_id(current_id.to_string());
            }
        }
    }

    /// Adds a predicate argument to the frame element list of this word.
    ///
    /// # Arguments
    ///
    /// * `predicate_id`: Synset id of this predicate.
    ///
    pub fn add_predicate(&mut self, predicate_id: String){
        self.frame_elements.push(FrameElement::new("PREDICATE".to_string(), "NONE".to_string(), predicate_id))
    }

    /// Removes the predicate.
    ///
    pub fn remove_predicate(&mut self){
        for i in 0..self.frame_elements.len() {
            if self.frame_elements[i].get_frame_element_type() == "PREDICATE".to_string(){
                self.frame_elements.remove(i);
                break;
            }
        }
    }

    /// Checks if one of the frame elements is a predicate.
    ///
    /// returns: bool True, if one of the frame elements is predicate; false otherwise.
    ///
    pub fn contains_predicate(&self) -> bool {
        for frame_element in self.frame_elements.iter() {
            if frame_element.get_frame_element_type() == "PREDICATE".to_string(){
                true;
            }
        }
        false
    }

    /// Checks if one of the frame element is a predicate with the given id.
    ///
    /// # Arguments
    ///
    /// * `predicate_id`: Synset id to check.
    ///
    /// returns: bool True, if one of the frame element is predicate; false otherwise.
    ///
    pub fn contains_predicate_with_id(&mut self, predicate_id: String) -> bool {
        for frame_element in self.frame_elements.iter_mut() {
            if frame_element.get_frame_element_type() == "PREDICATE".to_string() && frame_element.get_id() == predicate_id {
                true;
            }
        }
        false
    }

    /// Returns the first frame element whose id matches the given predicate id.
    ///
    /// # Arguments
    ///
    /// * `predicate_id`: id of the predicate to search for
    ///
    /// returns: Option<&FrameElement> the matching frame element, or None if no frame element has the given id
    ///
    pub fn get_frame_element_with_id(&self, predicate_id: String) -> Option<&FrameElement> {
        for frame_element in self.frame_elements.iter() {
            if frame_element.get_id() == predicate_id {
                return Some(frame_element);
            }
        }
        None
    }

    /// Returns the first non-predicate frame element whose id matches the given predicate id.
    ///
    /// # Arguments
    ///
    /// * `predicate_id`: id of the frame element to search for
    ///
    /// returns: Option<&FrameElement> the matching non-predicate frame element, or None if no matching non-predicate frame element exists
    ///
    pub fn get_non_predicate_frame_element_with_id(&self, predicate_id: String) -> Option<&FrameElement> {
        for frame_element in self.frame_elements.iter() {
            if frame_element.get_id() == predicate_id && frame_element.get_frame_element_type() != "PREDICATE".to_string(){
                return Some(frame_element);
            }
        }
        None
    }

    /// Returns the first non-predicate frame element whose id matches the given predicate id.
    ///
    /// returns: Vec<String> the matching non-predicate frame element, or None if no matching non-predicate frame element exists
    ///
    pub fn get_frame_elements(&self) -> Vec<String> {
        let mut result = Vec::new();
        for frame_element in self.frame_elements.iter() {
            result.push(frame_element.get_id().to_string());
        }
        result
    }

    /// Checks if the given argument with the given type and id exists or not.
    ///
    /// # Arguments
    ///
    /// * `frame_element_type`: Type of the frame element to search for.
    /// * `frame`: frame Name of the frame to search for
    /// * `id`: Id of the frame element to search for.
    ///
    /// returns: bool True if the frame element exists, false otherwise.
    ///
    pub fn contains_frame_element(&self, frame_element_type: String, frame: String, id: String) -> bool {
        for frame_element in self.frame_elements.iter() {
            if frame_element.get_frame_element_type() == frame_element_type && frame_element.get_id() == id && frame_element.get_frame() == frame{
                return true;
            }
        }
        false
    }
}