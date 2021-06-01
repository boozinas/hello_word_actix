table! {
    encuesta (id) {
        id -> Int4,
        descripcion -> Nullable<Text>,
        fecha_inicio -> Nullable<Date>,
        fecha_final -> Nullable<Date>,
        abierta -> Nullable<Bool>,
    }
}

table! {
    encuesta_pregunta (encuesta_id, pregunta_id) {
        encuesta_id -> Int4,
        pregunta_id -> Int4,
        id -> Int4,
    }
}

table! {
    encuesta_pregunta_respuesta (encuesta_pregunta_id, respuesta_id) {
        encuesta_pregunta_id -> Int4,
        respuesta_id -> Int4,
        id -> Int4,
    }
}

table! {
    pregunta (id) {
        id -> Int4,
        texto -> Nullable<Text>,
    }
}

table! {
    respuesta (encuesta_pregunta_respuesta_id, user_id) {
        encuesta_pregunta_respuesta_id -> Int4,
        user_id -> Int4,
        texto_otro -> Nullable<Text>,
    }
}

table! {
    respuesta_predefinida (id) {
        id -> Int4,
        opcion -> Nullable<Text>,
    }
}

table! {
    users (id) {
        name -> Nullable<Varchar>,
        geopoints -> Nullable<Varchar>,
        id -> Int4,
    }
}

joinable!(encuesta_pregunta -> encuesta (encuesta_id));
joinable!(encuesta_pregunta -> pregunta (pregunta_id));
joinable!(encuesta_pregunta_respuesta -> respuesta_predefinida (respuesta_id));

allow_tables_to_appear_in_same_query!(
    encuesta,
    encuesta_pregunta,
    encuesta_pregunta_respuesta,
    pregunta,
    respuesta,
    respuesta_predefinida,
    users,
);
