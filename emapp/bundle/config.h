/*
   Copyright (c) 2015-2021 hkrn All rights reserved

   This file is part of emapp component and it's licensed under Mozilla Public License. see LICENSE.md for more details.
 */

#ifndef EMBUNDLE_CONFIG_H_
#define EMBUNDLE_CONFIG_H_

#include "bx/macros.h"

#ifndef NDEBUG
#define BGFX_CONFIG_DEBUG 1
#else
#define BGFX_CONFIG_DEBUG 0
#endif

#define BGFX_CONFIG_USE_TINYSTL 1
#if defined(NANOEM_ENABLE_DEBUG_ALLOCATOR)
#define BGFX_CONFIG_MEMORY_TRACKING BX_CONFIG_SUPPORTS_THREADING
#endif /* NANOEM_ENABLE_DEBUG_ALLOCATOR */

#if BGFX_CONFIG_DEBUG
#undef BX_CHECK
#undef BX_TRACE
#undef BX_WARN
#undef BX_CONFIG_ALLOCATOR_DEBUG
#endif /* BGFX_CONFIG_DEBUG */

#define BGFX_CONFIG_DYNAMIC_INDEX_BUFFER_SIZE (4 << 22)
#define BGFX_CONFIG_TRANSIENT_INDEX_BUFFER_SIZE (2 << 20)
#define BGFX_CONFIG_DYNAMIC_VERTEX_BUFFER_SIZE (144 << 19)
#define BGFX_CONFIG_TRANSIENT_VERTEX_BUFFER_SIZE (2 << 22)
#define BGFX_CONFIG_MULTITHREADED 1
#define BGFX_CONFIG_RENDERER_NOOP 1

#if BX_PLATFORM_WINDOWS
#ifdef NANOEM_ENABLE_ANGLE_GLES
#define BGFX_CONFIG_RENDERER_OPENGL 0
#define BGFX_CONFIG_RENDERER_OPENGLES 30
#define BGFX_CONFIG_RENDERER_VULKAN 0
#else
#define BGFX_CONFIG_RENDERER_OPENGL 32
#define BGFX_CONFIG_RENDERER_OPENGLES 0
#define BGFX_CONFIG_RENDERER_VULKAN 1
#endif
#define BGFX_CONFIG_RENDERER_DIRECT3D9 1
#define BGFX_CONFIG_RENDERER_DIRECT3D11 1
#define BGFX_CONFIG_RENDERER_DIRECT3D12 1
#elif BX_PLATFORM_RPI
#define BGFX_CONFIG_RENDERER_OPENGLES 20
#define BGFX_CONFIG_RENDERER_OPENGL 0
#define EGL_CONTEXT_MAJOR_VERSION_KHR 0x3098
#define EGL_CONTEXT_MINOR_VERSION_KHR 0x30FB
#define EGL_CONTEXT_OPENGL_DEBUG_BIT_KHR 0x00000001
#define EGL_CONTEXT_FLAGS_KHR 0x30FC
#elif BX_PLATFORM_ANDROID || BX_PLATFORM_EMSCRIPTEN || BX_PLATFORM_NACL
#define BGFX_CONFIG_RENDERER_OPENGLES 30
#define BGFX_CONFIG_RENDERER_OPENGL 0
#define BGFX_CONFIG_RENDERER_VULKAN 0
#elif BX_PLATFORM_IOS
#define BGFX_CONFIG_RENDERER_OPENGLES 20
#define BGFX_CONFIG_RENDERER_OPENGL 0
#define BGFX_CONFIG_RENDERER_VULKAN 0
#elif BX_PLATFORM_OSX && BX_ARCH_64BIT
#define BGFX_CONFIG_RENDERER_OPENGLES 0
#define BGFX_CONFIG_RENDERER_OPENGL 32
#define BGFX_CONFIG_RENDERER_VULKAN 0
#else
#define BGFX_CONFIG_RENDERER_OPENGLES 0
#define BGFX_CONFIG_RENDERER_OPENGL 21
#define BGFX_CONFIG_RENDERER_VULKAN 0
#endif

#endif /* EMBUNDLE_CONFIG_H_ */
