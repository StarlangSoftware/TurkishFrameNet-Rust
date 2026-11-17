pub struct FrameElement {
    frame_element_type: String,
    frame: String,
    id: String
}

impl FrameElement {

    /// Constructor of {@link FrameElement} class which takes frameElementType and id as inputs and
    /// initializes corresponding attributes
    ///
    /// # Arguments
    ///
    /// * `frame_element_type`: Type of the frameElement
    /// * `frame`: Frame of the frameElement
    /// * `id`: Id of the frameElement
    ///
    /// returns: FrameElement
    ///
    pub fn new(frame_element_type: String, frame: String, id: String) -> FrameElement {
        Self{frame_element_type, frame, id}
    }

    /// A constructor of {@link FrameElement} class which takes frameElement string which is in the
    /// form of frameElementType$frame$id and parses this string into frameElementType, frame and id.
    /// If the frameElement string does not contain '$' then the
    /// constructor return a NONE type frameElement.
    ///
    /// # Arguments
    ///
    /// * `frame_element`: Frame element string containing the frameElementType, frame and id
    ///
    /// returns: FrameElement
    ///
    pub fn from_string(frame_element: String) -> FrameElement {
        if frame_element.contains("$"){
            let frame_element_type = &frame_element[..frame_element.find("$").unwrap()];
            let frame = &frame_element[frame_element.find("$").unwrap() + 1..frame_element.rfind("$").unwrap()];
            let id = &frame_element[frame_element.rfind("$").unwrap() + 1..frame_element.len()];
            FrameElement::new(frame_element_type.to_string(), frame.to_string(), id.to_string())
        } else {
            FrameElement{frame_element_type: "NONE".to_string(), frame: "".to_string(), id: "".to_string()}
        }
    }

    /// Accessor for frame_element_type.
    ///
    /// returns: String frame_element_type.
    ///
    pub fn get_frame_element_type(&self) -> &str {
        &self.frame_element_type
    }

    /// Accessor for frame.
    ///
    /// returns: String frame.
    ///
    pub fn get_frame(&self) -> &str {
        &self.frame
    }

    /// Accessor for id.
    ///
    /// returns: String id.
    ///
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Setter for the id
    ///
    /// # Arguments
    ///
    /// * `id`: New id of the frame element
    ///
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn to_string(&self) -> String {
        if self.frame_element_type == "NONE"{
            "NONE".to_string()
        } else {
            format!("{}${}${}", self.frame_element_type, self.frame, self.id)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::frame_element::FrameElement;

    #[test]
    fn test_frame_element() {
        let frame_element = FrameElement::from_string("Agent$Apply_Heat$TUR10-0100230".to_string());
        assert_eq!(frame_element.get_frame_element_type(), "Agent".to_string());
        assert_eq!(frame_element.get_frame(), "Apply_Heat".to_string());
        assert_eq!(frame_element.get_id(), "TUR10-0100230".to_string());
        assert_eq!(frame_element.to_string(), "Agent$Apply_Heat$TUR10-0100230".to_string());
    }
}