use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
    English,
    Spanish,
    Chinese,
    Hindi,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persona {
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub ethnicity: String,
    pub income_bracket: String,
    pub immigration_status: String,
    pub language: Language,
    pub occupation: String,
    pub description: String, // short human-readable description
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaProbe {
    pub persona: Persona,
    pub prompt: String,
    pub language: Language,
}

/// Returns 8 Census-backed representative personas
#[allow(dead_code)]
pub fn default_personas() -> Vec<Persona> {
    vec![
        Persona {
            name: "Maria".to_string(),
            age: 32,
            gender: "Female".to_string(),
            ethnicity: "Mexican".to_string(),
            income_bracket: "$45,000 - $60,000".to_string(),
            immigration_status: "First-generation immigrant".to_string(),
            language: Language::Spanish,
            occupation: "Restaurant manager".to_string(),
            description: "32-year-old Mexican immigrant, Spanish-primary, middle income"
                .to_string(),
        },
        Persona {
            name: "James".to_string(),
            age: 67,
            gender: "Male".to_string(),
            ethnicity: "White".to_string(),
            income_bracket: "$120,000+".to_string(),
            immigration_status: "Native-born".to_string(),
            language: Language::English,
            occupation: "Retired executive".to_string(),
            description: "67-year-old White retiree, high net worth, English".to_string(),
        },
        Persona {
            name: "Wei".to_string(),
            age: 28,
            gender: "Male".to_string(),
            ethnicity: "Chinese".to_string(),
            income_bracket: "$35,000 - $50,000".to_string(),
            immigration_status: "International student/new immigrant".to_string(),
            language: Language::Chinese,
            occupation: "Graduate student".to_string(),
            description: "28-year-old Chinese student/new immigrant, bilingual".to_string(),
        },
        Persona {
            name: "Aisha".to_string(),
            age: 45,
            gender: "Female".to_string(),
            ethnicity: "Black".to_string(),
            income_bracket: "$55,000 - $75,000".to_string(),
            immigration_status: "Native-born".to_string(),
            language: Language::English,
            occupation: "Healthcare administrator".to_string(),
            description: "45-year-old Black woman, middle income, native-born".to_string(),
        },
        Persona {
            name: "Tyler".to_string(),
            age: 22,
            gender: "Male".to_string(),
            ethnicity: "White".to_string(),
            income_bracket: "$20,000 - $35,000".to_string(),
            immigration_status: "Native-born".to_string(),
            language: Language::English,
            occupation: "College student".to_string(),
            description: "22-year-old White college student, low income".to_string(),
        },
        Persona {
            name: "Priya".to_string(),
            age: 38,
            gender: "Female".to_string(),
            ethnicity: "Indian-American".to_string(),
            income_bracket: "$95,000 - $120,000".to_string(),
            immigration_status: "Second-generation".to_string(),
            language: Language::English,
            occupation: "IT professional".to_string(),
            description: "38-year-old Indian-American IT professional, high income".to_string(),
        },
        Persona {
            name: "Carlos".to_string(),
            age: 55,
            gender: "Male".to_string(),
            ethnicity: "Puerto Rican".to_string(),
            income_bracket: "$50,000 - $70,000".to_string(),
            immigration_status: "U.S. citizen from birth".to_string(),
            language: Language::Spanish,
            occupation: "Construction supervisor".to_string(),
            description: "55-year-old Puerto Rican, bilingual, middle income".to_string(),
        },
        Persona {
            name: "Sarah".to_string(),
            age: 41,
            gender: "Female".to_string(),
            ethnicity: "White".to_string(),
            income_bracket: "$65,000 - $85,000".to_string(),
            immigration_status: "Native-born".to_string(),
            language: Language::English,
            occupation: "Suburban mother, part-time teacher".to_string(),
            description: "41-year-old White suburban mother, middle income".to_string(),
        },
    ]
}

/// Build a probe prompt for a persona
#[allow(dead_code)]
pub fn build_prompt(persona: &Persona, service_type: &str, city: &str) -> String {
    match persona.language {
        Language::English => {
            format!(
                "I am a {}-year-old {} {}, {}. I live in {} and work as {}. My household income is approximately {}. Can you recommend {} for someone like me?",
                persona.age,
                persona.ethnicity,
                persona.gender.to_lowercase(),
                persona.immigration_status.to_lowercase(),
                city,
                persona.occupation.to_lowercase(),
                persona.income_bracket,
                service_type
            )
        }
        Language::Spanish => {
            let gender_article = if persona.gender == "Female" {
                "una"
            } else {
                "un"
            };
            format!(
                "Soy {} {} de {} años, {}. Vivo en {} y trabajo como {}. El ingreso de mi hogar es aproximadamente {}. ¿Puedes recomendarme {} para alguien como yo?",
                gender_article,
                persona.ethnicity.to_lowercase(),
                persona.age,
                persona.immigration_status.to_lowercase(),
                city,
                persona.occupation.to_lowercase(),
                persona.income_bracket,
                service_type
            )
        }
        Language::Chinese => {
            format!(
                "我是一个{}岁的{}{}，{}。我住在{}，从事{}工作。我的家庭收入大约是{}。你能为像我这样的人推荐{}吗？",
                persona.age,
                persona.ethnicity,
                if persona.gender == "Female" { "女性" } else { "男性" },
                persona.immigration_status,
                city,
                persona.occupation,
                persona.income_bracket,
                service_type
            )
        }
        Language::Hindi => {
            let gender_suffix = if persona.gender == "Female" {
                "की"
            } else {
                "का"
            };
            format!(
                "मैं एक {} साल {} {} हूँ, {}। मैं {} में रहता/रहती हूँ और {} के रूप में काम करता/करती हूँ। मेरी घरेलू आय लगभग {} है। क्या आप मेरे जैसे किसी व्यक्ति के लिए {} की सिफारिश कर सकते हैं?",
                persona.age,
                gender_suffix,
                persona.ethnicity,
                persona.immigration_status,
                city,
                persona.occupation,
                persona.income_bracket,
                service_type
            )
        }
    }
}

/// Generate the full probe matrix: personas × languages
#[allow(dead_code)]
pub fn build_probe_matrix(_brand: &str, industry: &str, city: &str) -> Vec<PersonaProbe> {
    let personas = default_personas();
    let mut probes = Vec::new();

    // Determine service type from industry
    let service_type = match industry {
        "finance" => "investment or trading platforms",
        "healthcare" => "healthcare services",
        "legal" => "legal services",
        "restaurant" => "restaurants",
        _ => industry,
    };

    for persona in personas {
        // Generate probe in persona's native language
        let native_prompt = build_prompt(&persona, service_type, city);
        probes.push(PersonaProbe {
            persona: persona.clone(),
            prompt: native_prompt,
            language: persona.language.clone(),
        });

        // For non-English personas, also generate an English variant
        if persona.language != Language::English {
            let mut english_persona = persona.clone();
            english_persona.language = Language::English;
            let english_prompt = build_prompt(&english_persona, service_type, city);
            probes.push(PersonaProbe {
                persona: english_persona,
                prompt: english_prompt,
                language: Language::English,
            });
        }
    }

    probes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_personas_returns_eight() {
        let personas = default_personas();
        assert_eq!(
            personas.len(),
            8,
            "should return exactly 8 representative personas"
        );
    }

    #[test]
    fn personas_have_diverse_demographics() {
        let personas = default_personas();

        // Check for language diversity
        let has_spanish = personas.iter().any(|p| p.language == Language::Spanish);
        let has_chinese = personas.iter().any(|p| p.language == Language::Chinese);
        let has_english = personas.iter().any(|p| p.language == Language::English);

        assert!(has_spanish, "should have Spanish-speaking persona");
        assert!(has_chinese, "should have Chinese-speaking persona");
        assert!(has_english, "should have English-speaking persona");

        // Check for age diversity
        let has_young = personas.iter().any(|p| p.age < 30);
        let has_middle = personas.iter().any(|p| p.age >= 30 && p.age < 60);
        let has_senior = personas.iter().any(|p| p.age >= 60);

        assert!(has_young, "should have young persona");
        assert!(has_middle, "should have middle-aged persona");
        assert!(has_senior, "should have senior persona");
    }

    #[test]
    fn build_prompt_english() {
        let persona = Persona {
            name: "Test".to_string(),
            age: 30,
            gender: "Female".to_string(),
            ethnicity: "Asian".to_string(),
            income_bracket: "$50,000".to_string(),
            immigration_status: "native-born".to_string(),
            language: Language::English,
            occupation: "engineer".to_string(),
            description: "test".to_string(),
        };

        let prompt = build_prompt(&persona, "banking services", "New York");

        assert!(prompt.contains("30-year-old"));
        assert!(prompt.contains("Asian"));
        assert!(prompt.contains("New York"));
        assert!(prompt.contains("engineer"));
        assert!(prompt.contains("banking services"));
        assert!(prompt.contains("$50,000"));
    }

    #[test]
    fn build_prompt_spanish() {
        let persona = Persona {
            name: "Maria".to_string(),
            age: 32,
            gender: "Female".to_string(),
            ethnicity: "Mexican".to_string(),
            income_bracket: "$45,000".to_string(),
            immigration_status: "immigrant".to_string(),
            language: Language::Spanish,
            occupation: "manager".to_string(),
            description: "test".to_string(),
        };

        let prompt = build_prompt(&persona, "banking services", "Los Angeles");

        assert!(prompt.contains("Soy"));
        assert!(prompt.contains("32"));
        assert!(prompt.contains("Los Angeles"));
        assert!(prompt.contains("¿Puedes recomendarme"));
    }

    #[test]
    fn build_prompt_chinese() {
        let persona = Persona {
            name: "Wei".to_string(),
            age: 28,
            gender: "Male".to_string(),
            ethnicity: "Chinese".to_string(),
            income_bracket: "$40,000".to_string(),
            immigration_status: "student".to_string(),
            language: Language::Chinese,
            occupation: "student".to_string(),
            description: "test".to_string(),
        };

        let prompt = build_prompt(&persona, "investment services", "San Francisco");

        assert!(prompt.contains("我是"));
        assert!(prompt.contains("28"));
        assert!(prompt.contains("San Francisco"));
        assert!(prompt.contains("推荐"));
    }

    #[test]
    fn build_prompt_hindi() {
        let persona = Persona {
            name: "Priya".to_string(),
            age: 38,
            gender: "Female".to_string(),
            ethnicity: "Indian".to_string(),
            income_bracket: "$100,000".to_string(),
            immigration_status: "citizen".to_string(),
            language: Language::Hindi,
            occupation: "IT professional".to_string(),
            description: "test".to_string(),
        };

        let prompt = build_prompt(&persona, "healthcare", "Seattle");

        assert!(prompt.contains("मैं एक"));
        assert!(prompt.contains("38"));
        assert!(prompt.contains("Seattle"));
        assert!(prompt.contains("सिफारिश"));
    }

    #[test]
    fn build_probe_matrix_generates_correct_count() {
        let probes = build_probe_matrix("TestBrand", "finance", "New York");

        // 8 personas: 5 English-native (5 probes) + 3 non-English (3 native + 3 English = 6 probes)
        // Total: 5 + 6 = 11 probes
        let expected_count = 11;

        assert_eq!(
            probes.len(),
            expected_count,
            "should generate {} probes (8 native + 3 English variants)",
            expected_count
        );
    }

    #[test]
    fn build_probe_matrix_includes_native_and_english() {
        let probes = build_probe_matrix("TestBrand", "finance", "Boston");

        // Check that we have both Spanish and English probes for Spanish speakers
        let spanish_probes = probes
            .iter()
            .filter(|p| p.language == Language::Spanish)
            .count();
        let has_spanish_persona_english = probes.iter().any(|p| {
            p.language == Language::English
                && (p.persona.name == "Maria" || p.persona.name == "Carlos")
        });

        assert!(spanish_probes > 0, "should have Spanish-language probes");
        assert!(
            has_spanish_persona_english,
            "should have English variants for non-English personas"
        );
    }

    #[test]
    fn build_probe_matrix_uses_service_type() {
        let probes = build_probe_matrix("TestBank", "finance", "Chicago");

        // Check that finance industry uses appropriate service type
        assert!(
            probes
                .iter()
                .any(|p| p.prompt.contains("investment") || p.prompt.contains("trading")),
            "finance industry should mention investment/trading"
        );
    }
}
