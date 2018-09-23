use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    translations: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        if let Some(protein) = self.translations.get(codon) {
            return Some(protein);
        }
        None
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let maybe_proteins = (0..rna.len())
            .step_by(3)
            .map(|i| self.name_for(&rna[i..i + 3]))
            .collect::<Vec<_>>();

        if maybe_proteins.iter().any(|p| p.is_none()) {
            return None;
        }

        let proteins: Vec<_> = maybe_proteins.iter().map(|p| p.unwrap()).collect();

        if let Some(stop_position) = proteins.iter().position(|&p| p == "stop codon") {
            return Some(proteins[..stop_position].to_vec());
        }

        Some(proteins.to_vec())
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut translations = HashMap::new();

    for (protein, codon) in pairs {
        translations.insert(protein, codon);
    }

    CodonsInfo { translations }
}
