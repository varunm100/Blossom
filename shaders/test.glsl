#version 460
#pragma shader_stage(vertex)

#extension GL_EXT_buffer_reference : require
#extension GL_EXT_buffer_reference2 : require
#extension GL_ARB_gpu_shader_int64 : require

struct BufferView {
  uint64_t ptr;
  uint size;
};

layout(buffer_reference, std430) buffer VertexArray {
  vec3 vertices[];
};

layout(binding=0) buffer ResourceHeap {
  BufferView buffers[];
} resource_heap;

layout(push_constant) uniform PushConstants {
  uint vbo_index;
} push_constants;

void main() {
  VertexArray array = VertexArray(resource_heap.buffers[push_constants.vbo_index].ptr);
  vec3 vert = array.vertices[gl_VertexIndex];
  gl_Position = vec4(vert, 1.);
}
