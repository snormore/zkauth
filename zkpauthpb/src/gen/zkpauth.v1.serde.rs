// @generated
impl serde::Serialize for AuthenticationAnswerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.auth_id.is_empty() {
            len += 1;
        }
        if !self.s.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zkpauth.v1.AuthenticationAnswerRequest", len)?;
        if !self.auth_id.is_empty() {
            struct_ser.serialize_field("authId", &self.auth_id)?;
        }
        if !self.s.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("s", pbjson::private::base64::encode(&self.s).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationAnswerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_id",
            "authId",
            "s",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthId,
            S,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authId" | "auth_id" => Ok(GeneratedField::AuthId),
                            "s" => Ok(GeneratedField::S),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationAnswerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zkpauth.v1.AuthenticationAnswerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthenticationAnswerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_id__ = None;
                let mut s__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthId => {
                            if auth_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authId"));
                            }
                            auth_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::S => {
                            if s__.is_some() {
                                return Err(serde::de::Error::duplicate_field("s"));
                            }
                            s__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AuthenticationAnswerRequest {
                    auth_id: auth_id__.unwrap_or_default(),
                    s: s__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zkpauth.v1.AuthenticationAnswerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticationAnswerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.session_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zkpauth.v1.AuthenticationAnswerResponse", len)?;
        if !self.session_id.is_empty() {
            struct_ser.serialize_field("sessionId", &self.session_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationAnswerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_id",
            "sessionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sessionId" | "session_id" => Ok(GeneratedField::SessionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationAnswerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zkpauth.v1.AuthenticationAnswerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthenticationAnswerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SessionId => {
                            if session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionId"));
                            }
                            session_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AuthenticationAnswerResponse {
                    session_id: session_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zkpauth.v1.AuthenticationAnswerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticationChallengeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.r1.is_empty() {
            len += 1;
        }
        if !self.r2.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zkpauth.v1.AuthenticationChallengeRequest", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.r1.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("r1", pbjson::private::base64::encode(&self.r1).as_str())?;
        }
        if !self.r2.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("r2", pbjson::private::base64::encode(&self.r2).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationChallengeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "r1",
            "r2",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            R1,
            R2,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "r1" => Ok(GeneratedField::R1),
                            "r2" => Ok(GeneratedField::R2),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationChallengeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zkpauth.v1.AuthenticationChallengeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthenticationChallengeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut r1__ = None;
                let mut r2__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::R1 => {
                            if r1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("r1"));
                            }
                            r1__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::R2 => {
                            if r2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("r2"));
                            }
                            r2__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AuthenticationChallengeRequest {
                    user: user__.unwrap_or_default(),
                    r1: r1__.unwrap_or_default(),
                    r2: r2__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zkpauth.v1.AuthenticationChallengeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthenticationChallengeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.auth_id.is_empty() {
            len += 1;
        }
        if !self.c.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zkpauth.v1.AuthenticationChallengeResponse", len)?;
        if !self.auth_id.is_empty() {
            struct_ser.serialize_field("authId", &self.auth_id)?;
        }
        if !self.c.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("c", pbjson::private::base64::encode(&self.c).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticationChallengeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_id",
            "authId",
            "c",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthId,
            C,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authId" | "auth_id" => Ok(GeneratedField::AuthId),
                            "c" => Ok(GeneratedField::C),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthenticationChallengeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zkpauth.v1.AuthenticationChallengeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthenticationChallengeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_id__ = None;
                let mut c__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthId => {
                            if auth_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authId"));
                            }
                            auth_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::C => {
                            if c__.is_some() {
                                return Err(serde::de::Error::duplicate_field("c"));
                            }
                            c__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AuthenticationChallengeResponse {
                    auth_id: auth_id__.unwrap_or_default(),
                    c: c__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zkpauth.v1.AuthenticationChallengeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPublicParametersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zkpauth.v1.GetPublicParametersRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPublicParametersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPublicParametersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zkpauth.v1.GetPublicParametersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPublicParametersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetPublicParametersRequest {
                })
            }
        }
        deserializer.deserialize_struct("zkpauth.v1.GetPublicParametersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPublicParametersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.p.is_empty() {
            len += 1;
        }
        if !self.q.is_empty() {
            len += 1;
        }
        if !self.g.is_empty() {
            len += 1;
        }
        if !self.h.is_empty() {
            len += 1;
        }
        if self.bit_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zkpauth.v1.GetPublicParametersResponse", len)?;
        if !self.p.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("p", pbjson::private::base64::encode(&self.p).as_str())?;
        }
        if !self.q.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("q", pbjson::private::base64::encode(&self.q).as_str())?;
        }
        if !self.g.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("g", pbjson::private::base64::encode(&self.g).as_str())?;
        }
        if !self.h.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("h", pbjson::private::base64::encode(&self.h).as_str())?;
        }
        if self.bit_size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("bitSize", ToString::to_string(&self.bit_size).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPublicParametersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "p",
            "q",
            "g",
            "h",
            "bit_size",
            "bitSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            P,
            Q,
            G,
            H,
            BitSize,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "p" => Ok(GeneratedField::P),
                            "q" => Ok(GeneratedField::Q),
                            "g" => Ok(GeneratedField::G),
                            "h" => Ok(GeneratedField::H),
                            "bitSize" | "bit_size" => Ok(GeneratedField::BitSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPublicParametersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zkpauth.v1.GetPublicParametersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPublicParametersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut p__ = None;
                let mut q__ = None;
                let mut g__ = None;
                let mut h__ = None;
                let mut bit_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::P => {
                            if p__.is_some() {
                                return Err(serde::de::Error::duplicate_field("p"));
                            }
                            p__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Q => {
                            if q__.is_some() {
                                return Err(serde::de::Error::duplicate_field("q"));
                            }
                            q__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::G => {
                            if g__.is_some() {
                                return Err(serde::de::Error::duplicate_field("g"));
                            }
                            g__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::H => {
                            if h__.is_some() {
                                return Err(serde::de::Error::duplicate_field("h"));
                            }
                            h__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BitSize => {
                            if bit_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitSize"));
                            }
                            bit_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetPublicParametersResponse {
                    p: p__.unwrap_or_default(),
                    q: q__.unwrap_or_default(),
                    g: g__.unwrap_or_default(),
                    h: h__.unwrap_or_default(),
                    bit_size: bit_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zkpauth.v1.GetPublicParametersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.y1.is_empty() {
            len += 1;
        }
        if !self.y2.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("zkpauth.v1.RegisterRequest", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.y1.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("y1", pbjson::private::base64::encode(&self.y1).as_str())?;
        }
        if !self.y2.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("y2", pbjson::private::base64::encode(&self.y2).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "y1",
            "y2",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Y1,
            Y2,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "y1" => Ok(GeneratedField::Y1),
                            "y2" => Ok(GeneratedField::Y2),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zkpauth.v1.RegisterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut y1__ = None;
                let mut y2__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Y1 => {
                            if y1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("y1"));
                            }
                            y1__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Y2 => {
                            if y2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("y2"));
                            }
                            y2__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RegisterRequest {
                    user: user__.unwrap_or_default(),
                    y1: y1__.unwrap_or_default(),
                    y2: y2__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("zkpauth.v1.RegisterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("zkpauth.v1.RegisterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct zkpauth.v1.RegisterResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RegisterResponse {
                })
            }
        }
        deserializer.deserialize_struct("zkpauth.v1.RegisterResponse", FIELDS, GeneratedVisitor)
    }
}
