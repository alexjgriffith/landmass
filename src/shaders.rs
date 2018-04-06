pub static VERTEX_SHADER_SRC: &str = r#"
        #version 150
        in vec3 position;
        in vec3 normal;
        flat out vec3 v_normal;
        out vec3 v_position;
        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;
        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position =   perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;

pub static FRAGMENT_SHADER_SRC: &str = r#"
        #version 150
        flat in vec3 v_normal;
        in vec3 v_position;

        out vec4 color;
        const vec3 ambient_color = vec3(0.1, 0.0, 0.1);
        const vec3 diffuse_color = vec3(0.3, 0.05, 0.1);
        const vec3 specular_color = vec3(1.0, 1.0, 1.0);        
        uniform vec3 u_light;
        void main() {
             float diffuse = max(dot(normalize(v_normal), normalize(u_light)),0.0);
             vec3 camera_dir = normalize(-v_position);
             vec3 half_direction = normalize(normalize(u_light)+camera_dir);
             float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
             color = vec4(ambient_color+diffuse*diffuse_color + specular*specular_color, 1.0);
        }
    "#;
