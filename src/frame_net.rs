use crate::frame::Frame;
use nlptoolkit_xmlparser::xml_document::XmlDocument;

pub struct Framenet{
    frames: Vec<Frame>
}

impl Framenet{
    pub fn new() -> Framenet{
        let mut frames: Vec<Frame> = Vec::new();
        let mut doc: XmlDocument = XmlDocument::new("framenet.xml".to_string());
        doc.parse();
        let framenet = doc.get_first_child().unwrap();
        let mut current_outer = framenet.get_first_child();
        while let Some(frame_node_index) = current_outer {
            let frame_node = doc.get_xml_element(frame_node_index);
            let mut current_frame = Frame::new(frame_node.get_attribute_value("NAME".to_string()).unwrap().to_string());
            let lexical_units = doc.get_xml_element(frame_node.get_first_child().unwrap());
            let mut current_inner1 = lexical_units.get_first_child();
            while let Some(lexical_unit_index) = current_inner1 {
                let lexical_unit = doc.get_xml_element(lexical_unit_index);
                current_frame.add_lexical_unit(lexical_unit.get_pc_data().to_string());
                current_inner1 = lexical_unit.get_next_sibling();
            }
            let frame_elements = doc.get_xml_element(lexical_units.get_next_sibling().unwrap());
            let mut current_inner2 = frame_elements.get_first_child();
            while let Some(frame_element_index) = current_inner2 {
                let frame_element = doc.get_xml_element(frame_element_index);
                current_frame.add_frame_element(frame_element.get_pc_data().to_string());
                current_inner2 = frame_element.get_next_sibling();
            }
            frames.push(current_frame);
            current_outer = frame_node.get_next_sibling();
        }
        Self{frames}
    }

    /// Checks if the given lexical unit exists in any frame in the frame set.
    ///
    /// # Arguments
    ///
    /// * `synset_id`: Id of the lexical unit
    ///
    /// returns: bool True if any frame contains the given lexical unit, false otherwise.
    ///
    pub fn lexical_units(&self, synset_id: &str) -> bool{
        for frame in &self.frames{
            if frame.lexical_unit_exists(synset_id){
                return true;
            }
        }
        false
    }

    /// Returns an array of frames that contain the given lexical unit in their lexical units
    ///
    /// # Arguments
    ///
    /// * `synset_id`: Id of the lexical unit.
    ///
    /// returns: Vec<&Frame, Global> An array of frames that contains the given lexical unit.
    ///
    pub fn get_frames(&self, synset_id: &str) -> Vec<&Frame> {
        let mut result : Vec<&Frame> = Vec::new();
        for frame in &self.frames{
            if frame.lexical_unit_exists(synset_id){
                result.push(frame);
            }
        }
        result
    }

    pub fn size(&self) -> usize{
        self.frames.len()
    }

    /// Returns the element at the specified position in the frame list.
    ///
    /// # Arguments
    ///
    /// * `index`: index of the element to return
    ///
    /// returns: &Frame The element at the specified position in the frame list.
    ///
    pub fn get_frame(&self, index: usize) -> &Frame{
        &self.frames[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::frame_net::Framenet;

    #[test]
    fn test_frame_size() {
        let framenet = Framenet::new();
        assert_eq!(809, framenet.size());
    }

    #[test]
    fn test_lexical_unit_size() {
        let framenet = Framenet::new();
        let mut total = 0;
        for i in 0..framenet.size(){
            total += framenet.get_frame(i).lexical_unit_size();
        }
        assert_eq!(8484, total);
    }

    #[test]
    fn test_frame_element_size() {
        let framenet = Framenet::new();
        let mut total = 0;
        for i in 0..framenet.size(){
            total += framenet.get_frame(i).frame_elements_size();
        }
        assert_eq!(8656, total);
    }

}