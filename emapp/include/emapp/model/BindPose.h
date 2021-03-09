/*
   Copyright (c) 2015-2021 hkrn All rights reserved

   This file is part of emapp component and it's licensed under Mozilla Public License. see LICENSE.md for more details.
 */

#pragma once
#ifndef NANOEM_EMAPP_MODEL_BINDPOSE_H_
#define NANOEM_EMAPP_MODEL_BINDPOSE_H_

#include "emapp/Forward.h"

namespace nanoem {

class Error;
class IWriter;
class Model;
class URI;

namespace model {

struct BindPose {
    static StringList loadableExtensions();
    static StringSet loadableExtensionsSet();
    static bool isLoadableExtension(const String &extension);
    static bool isLoadableExtension(const URI &fileURI);
    static void releaseAllMessages(void *states, size_t numStates);

    BindPose();
    BindPose(const BindPose &value);
    ~BindPose() NANOEM_DECL_NOEXCEPT;

    bool load(Model *model, const nanoem_u8_t *data, nanoem_rsize_t size, Error &error) const;
    bool load(Model *model, const ByteArray &bytes, Error &error) const;
    bool save(const Model *model, IWriter *writer, Error &error) const;
    void saveAllMessages(void *states, size_t &numStates) const;
    void restoreAllMessages(const Model *model, void *states, size_t numStates);

    struct Parameter {
        Parameter();
        Parameter(const Parameter &value);
        void save(const nanoem_model_bone_t *bonePtr);
        void restore(const nanoem_model_bone_t *bonePtr) const;
        const nanoem_model_bone_t *m_bonePtr;
        Vector3 m_localMorphTranslation;
        Vector3 m_localUserTranslation;
        Quaternion m_localMorphOrientation;
        Quaternion m_localUserOrientation;
        glm::u8vec4 m_bezierControlPoints[NANOEM_MOTION_BONE_KEYFRAME_INTERPOLATION_TYPE_MAX_ENUM];
    };
    tinystl::vector<Parameter, TinySTLAllocator> m_parameters;
};

} /* namespace model */
} /* namespace nanoem */

#endif /* NANOEM_EMAPP_MODEL_BINDPOSE_H_ */
