# dd-gui
Minimal, hardware accelerated immediate mode OpenGL GUI library in rust.

The first goal of this library will be to support VST development, but there's no reason it can't be used for other purposes. It directly uses a glium surface, so can draw on top of glium based games, etc.

Short term roadmap:
- [x] renderer: rect
- [x] renderer: circle
- [ ] renderer: arc
- [ ] renderer: text
- [ ] renderer: image
- [x] ui: drag and drop (added as a PR to winit core)
- [x] ui: mouse state (move, click, drag)
- [ ] styles: defaults
- [ ] styles: borders for all shapes
- [ ] widgets: knob
- [x] widgets: button
- [ ] widgets: slider
