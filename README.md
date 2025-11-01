# Proyecto 3: Sistema Solar - Software Renderer

## Descripción del Proyecto

Este proyecto consiste en la creación de un sistema solar interactivo utilizando un software renderer desarrollado desde cero en Rust. El proyecto utiliza técnicas de renderizado 3D, iluminación, y carga de modelos OBJ para crear una experiencia visual inmersiva.

## Características Actuales

### ✅ Nave Espacial (30 puntos)
Se ha modelado y renderizado una nave espacial personalizada que servirá como cámara móvil para explorar el sistema solar.

![Nave Espacial Jett](spaceship/assets/spaceship_image.png)

**Características de la nave:**
- Modelo 3D personalizado creado en Blender
- Múltiples materiales y colores:
  - Cuerpo azul (Material.001)
  - Cabina celeste (Material.002)
  - Misiles/armas gris (Material.003)
- Sistema de iluminación bidireccional con múltiples fuentes de luz
- Carga de archivos OBJ con materiales MTL

### Controles Actuales

- ⌨️ **ESPACIO**: Activar/desactivar rotación automática
- ⌨️ **E/R**: Rotar en eje Y
- ⌨️ **Q/W**: Rotar en eje X
- ⌨️ **T/Y**: Rotar en eje Z
- ⌨️ **Flechas**: Mover el modelo
- ⌨️ **A/S**: Zoom
- 🖱️ **Rueda del mouse**: Zoom
- ⌨️ **ESC**: Salir

## Características Planeadas

### Sistema Solar
- [ ] Renderizar un sol central
- [ ] Múltiples planetas en órbitas circulares
- [ ] Rotación de planetas sobre su eje
- [ ] Traslación de planetas en el plano eclíptico
- [ ] Lunas orbitando planetas

### Cámara y Navegación
- [ ] Cámara móvil en el plano eclíptico
- [ ] Movimiento 3D para la cámara (40 puntos)
- [ ] Instant warping a diferentes puntos (10 puntos)
- [ ] Efecto de warp animado (10 puntos adicionales)
- [ ] Nave que sigue a la cámara (✅ 30 puntos - Completado)

### Extras
- [ ] Skybox con estrellas (10 puntos)
- [ ] Detección de colisiones (10 puntos)
- [ ] Renderizado de órbitas planetarias (20 puntos)

## Tecnologías Utilizadas

- **Lenguaje**: Rust
- **Matemáticas**: nalgebra-glm
- **Ventanas**: minifb
- **Carga de modelos**: tobj
- **Modelado 3D**: Blender

## Estructura del Proyecto

```
spaceship/
├── src/
│   ├── main.rs          # Punto de entrada y loop principal
│   ├── framebuffer.rs   # Buffer de renderizado
│   ├── triangle.rs      # Rasterización de triángulos
│   ├── vertex.rs        # Estructura de vértices
│   ├── obj.rs           # Carga de modelos OBJ
│   ├── color.rs         # Manejo de colores
│   ├── fragment.rs      # Fragmentos para rasterización
│   └── shaders.rs       # Vertex shader
├── assets/
│   ├── Jett.obj         # Modelo 3D de la nave
│   ├── Jett.mtl         # Materiales de la nave
│   └── spaceship_image.png  # Imagen de la nave
└── Cargo.toml           # Dependencias del proyecto
```

## Instalación y Ejecución

### Requisitos
- Rust (versión 1.70 o superior)
- Cargo

### Compilar y Ejecutar
```bash
cargo run --release
```

## Progreso del Proyecto

### Fase 1: Nave Espacial ✅
- [x] Modelado de la nave en Blender
- [x] Exportación a formato OBJ con materiales
- [x] Carga y renderizado de la nave
- [x] Sistema de iluminación
- [x] Controles básicos de cámara

### Fase 2: Sistema Solar (En Progreso)
- [ ] Implementación del sol
- [ ] Creación de planetas
- [ ] Sistema de órbitas
- [ ] Rotación y traslación planetaria

### Fase 3: Cámara y Navegación (Pendiente)
- [ ] Sistema de cámara libre
- [ ] Movimiento en 3D
- [ ] Sistema de warp

### Fase 4: Extras (Pendiente)
- [ ] Skybox
- [ ] Colisiones
- [ ] Órbitas visuales

## Autor

**Proyecto desarrollado para el curso de Gráficas por Computadora**
- Universidad del Valle de Guatemala
- Fernando Rueda - 23748
- 2025

## Video Demostración

_[Video pendiente]_

---

**Nota**: Este proyecto está en desarrollo activo. Las características marcadas con ✅ están completadas, mientras que las marcadas con [ ] están pendientes de implementación.
