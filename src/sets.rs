use std::collections::HashSet;
use crate::elements::Elements;
use crate::error::{SetError, SetErrorType};

pub struct Set {
    pub elements: HashSet<Elements>,
}

impl Set {
    pub fn SetfromHashSet(elements: HashSet<Elements>) -> Self {
        Self { elements }
    }
}

impl SetOperations for Set {
    fn add(&mut self, elements: &[Elements]) -> Result<(), SetError> {
        for e in elements {
            if !self.elements.contains(e) {
                self.elements.insert(e.clone());
            } else {
                return Err(SetError::new(
                    "Set cannot contain multiple elements".to_string(),
                    SetErrorType::DuplicateElement,
                ));
            }
        }
        Ok(())
    }

    fn remove(&mut self, elements: &[Elements]) -> Result<(), SetError> {
        for e in elements {
            if !self.elements.remove(e) {
                return Err(SetError::new(
                    "Failed to remove element from set".to_string(),
                    SetErrorType::FailedToRemoveElement,
                ));
            }
        }
        Ok(())
    }

    fn union(&self, set: &Set) -> Result<Set, SetError> {
        let union = self.elements.union(&set.elements);

        let new: HashSet<Elements> = union.map(|x| x.clone()).collect();

        Ok(Set::SetfromHashSet(new))
    }

    fn intersection(&self, set: &Set) -> Result<Set, SetError> {
        let intersection = set.elements.intersection(&self.elements);

        let new: HashSet<Elements> = intersection.map(|x| x.clone()).collect();

        Ok(Set::SetfromHashSet(new))
    }

    fn difference(&self, set: &Set) -> Result<Set, SetError> {
        let difference = set.elements.difference(&self.elements);

        let new: HashSet<Elements> = difference.map(|x| x.clone()).collect();

        Ok(Set::SetfromHashSet(new))
    }
}

pub trait SetOperations {
    //add an element to an set
    fn add(&mut self, element: &[Elements]) -> Result<(), SetError>;
    // remove an element of an set
    fn remove(&mut self, elements: &[Elements]) -> Result<(), SetError>;

    //Operations that create another set
    fn union(&self, set: &Set) -> Result<Set, SetError>;
    fn intersection(&self, set: &Set) -> Result<Set, SetError>;
    fn difference(&self, set: &Set) -> Result<Set, SetError>;
}