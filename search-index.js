var searchIndex = {};
searchIndex["rugl"] = {"doc":"","items":[[4,"Primitive","rugl","",null,null],[13,"Points","","",0,null],[13,"Lines","","",0,null],[13,"LineStrip","","",0,null],[13,"LineLoop","","",0,null],[13,"Triangles","","",0,null],[13,"TriangleStrip","","",0,null],[13,"TriangleFan","","",0,null],[0,"draw_builder","","",null,null],[3,"DrawConfig","rugl::draw_builder","",null,null],[12,"vert","","",1,null],[12,"frag","","",1,null],[12,"attributes","","",1,null],[12,"elements","","",1,null],[12,"uniform_setters","","",1,null],[12,"primitive","","",1,null],[12,"count","","",1,null],[3,"DrawBuilder","","",null,null],[12,"config","","",2,null],[11,"new","","",2,{"inputs":[],"output":{"name":"drawbuilder"}}],[11,"vert","","",2,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"drawbuilder"}}],[11,"frag","","",2,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"drawbuilder"}}],[11,"uniform","","",2,{"inputs":[{"name":"self"},{"name":"str"},{"name":"box"}],"output":{"name":"drawbuilder"}}],[11,"attribute","","",2,{"inputs":[{"name":"self"},{"name":"str"},{"name":"bufferabledata"}],"output":{"name":"drawbuilder"}}],[11,"primitive","","",2,{"inputs":[{"name":"self"},{"name":"primitive"}],"output":{"name":"drawbuilder"}}],[11,"elements","","",2,{"inputs":[{"name":"self"},{"name":"bufferableelementsdata"}],"output":{"name":"drawbuilder"}}],[11,"count","","",2,{"inputs":[{"name":"self"},{"name":"i32"}],"output":{"name":"drawbuilder"}}],[11,"finalize","","",2,{"inputs":[{"name":"self"}],"output":{"name":"box"}}],[0,"clear","rugl","",null,null],[3,"Clear","rugl::clear","`Clear` combines `glClearColor`, `glClearDepth`, `glClearStencil` and `glClear` into a single procedure, which has the following default usage:",null,null],[12,"color","","Sets the clear color",3,null],[12,"depth","","Sets the clear depth value",3,null],[12,"stencil","","Sets the clear stencil value",3,null],[11,"new","","Create a new clear object.",3,{"inputs":[],"output":{"name":"clear"}}],[11,"execute","","Execute the glClear with the set values.",3,{"inputs":[{"name":"self"}],"output":null}],[11,"make_execute_fn","","Consume the struct and get a closure over `execute()`.",3,{"inputs":[{"name":"self"}],"output":{"name":"box"}}],[0,"gl_helpers","rugl","",null,null],[3,"AttributeInfo","rugl::gl_helpers","",null,null],[12,"name","","",4,null],[12,"index","","",4,null],[12,"type_enum","","",4,null],[12,"data_type","","",4,null],[12,"data_size","","",4,null],[3,"UniformInfo","","",null,null],[12,"name","","",5,null],[12,"index","","",5,null],[12,"location","","",5,null],[12,"data_type","","",5,null],[12,"data_size","","",5,null],[5,"compile_shader","","",null,{"inputs":[{"name":"str"},{"name":"glenum"}],"output":{"name":"gluint"}}],[5,"link_program","","",null,{"inputs":[{"name":"gluint"},{"name":"gluint"}],"output":{"name":"gluint"}}],[5,"use_program","","",null,{"inputs":[{"name":"gluint"}],"output":null}],[5,"create_buffer","","",null,null],[5,"create_buffer_u32","","",null,null],[5,"create_vao","","",null,{"inputs":[],"output":{"name":"gluint"}}],[5,"bind_vao","","",null,{"inputs":[{"name":"gluint"}],"output":null}],[5,"bind_attribute_buffer","","",null,{"inputs":[{"name":"gluint"},{"name":"attributeinfo"}],"output":null}],[5,"get_attribute_count","","",null,{"inputs":[{"name":"gluint"}],"output":{"name":"glint"}}],[5,"get_uniform_count","","",null,{"inputs":[{"name":"gluint"}],"output":{"name":"glint"}}],[5,"get_attribute_info","","",null,{"inputs":[{"name":"gluint"},{"name":"glint"}],"output":{"name":"attributeinfo"}}],[5,"get_uniform_info","","",null,{"inputs":[{"name":"gluint"},{"name":"glint"}],"output":{"name":"uniforminfo"}}],[5,"get_program_attributes","","",null,{"inputs":[{"name":"gluint"}],"output":{"name":"vec"}}],[5,"get_uniforms","","",null,{"inputs":[{"name":"gluint"}],"output":{"name":"vec"}}],[5,"get_attribute_type_size","","",null,{"inputs":[{"name":"glenum"}],"output":{"name":"glint"}}],[5,"get_attribute_type","","",null,{"inputs":[{"name":"glenum"}],"output":{"name":"glenum"}}],[5,"draw_arrays","","",null,{"inputs":[{"name":"glenum"},{"name":"glint"},{"name":"glsizei"}],"output":null}],[5,"draw_elements","","",null,{"inputs":[{"name":"glenum"},{"name":"glsizei"}],"output":null}],[5,"get_major_version","","",null,{"inputs":[],"output":{"name":"i32"}}],[5,"get_minor_version","","",null,{"inputs":[],"output":{"name":"i32"}}],[5,"gl_attribute_enum_to_string","","",null,{"inputs":[{"name":"glenum"}],"output":{"name":"str"}}],[5,"gl_draw_mode_enum_to_string","","",null,{"inputs":[{"name":"glenum"}],"output":{"name":"str"}}],[5,"gl_shader_type_enum_to_string","","",null,{"inputs":[{"name":"glenum"}],"output":{"name":"str"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",5,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"rugl","rugl","",null,null],[3,"Environment","rugl::rugl","",null,null],[12,"time","","",6,null],[12,"tick","","",6,null],[12,"viewport_width","","",6,null],[12,"viewport_height","","",6,null],[3,"Rugl","","",null,null],[5,"init","","",null,{"inputs":[],"output":{"name":"rugl"}}],[5,"init_headless","","",null,{"inputs":[],"output":{"name":"rugl"}}],[11,"draw","","",7,{"inputs":[{"name":"self"}],"output":{"name":"drawbuilder"}}],[11,"clear","","",7,{"inputs":[{"name":"self"}],"output":{"name":"clear"}}],[11,"frame","","",7,{"inputs":[{"name":"self"},{"name":"f"}],"output":null}],[0,"buffers","rugl","",null,null],[8,"BufferableData","rugl::buffers","",null,null],[10,"to_buffer","","",8,{"inputs":[{"name":"self"}],"output":{"name":"gluint"}}],[8,"BufferableElementsData","","Enumerate options for buffers used as the elements in gl::DrawElements().",null,null],[10,"to_buffer","","",9,{"inputs":[{"name":"self"},{"name":"primitive"}],"output":{"name":"gluint"}}],[10,"get_count","","",9,{"inputs":[{"name":"self"},{"name":"primitive"}],"output":{"name":"glint"}}],[0,"uniforms","rugl","",null,null],[8,"UniformValue","rugl::uniforms","",null,null],[10,"set_uniform","","",10,{"inputs":[{"name":"self"},{"name":"glint"},{"name":"glenum"},{"name":"i32"}],"output":null}],[11,"to_gl_enum","rugl","",0,{"inputs":[{"name":"self"}],"output":{"name":"glenum"}}],[14,"rugl","","",null,null]],"paths":[[4,"Primitive"],[3,"DrawConfig"],[3,"DrawBuilder"],[3,"Clear"],[3,"AttributeInfo"],[3,"UniformInfo"],[3,"Environment"],[3,"Rugl"],[8,"BufferableData"],[8,"BufferableElementsData"],[8,"UniformValue"]]};
initSearch(searchIndex);