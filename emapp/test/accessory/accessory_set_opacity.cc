/*
   Copyright (c) 2015-2021 hkrn All rights reserved

   This file is part of emapp component and it's licensed under Mozilla Public License. see LICENSE.md for more details.
 */

#include "../common.h"

#include "emapp/Accessory.h"
#include "emapp/internal/AccessoryValueState.h"

using namespace nanoem;
using namespace nanoem::internal;
using namespace test;

TEST_CASE("accessory_set_opacity", "[emapp][accessory]")
{
    TestScope scope;
    {
        ProjectPtr o = scope.createProject();
        Project *project = o.get()->m_project;
        Accessory *accessory = o->createAccessory();
        accessory->setTranslation(Vector3(1, 2, 3));
        accessory->setOrientation(glm::radians(Vector3(18, 45, 73)));
        accessory->setOpacity(0.4f);
        accessory->setScaleFactor(0.9f);
        project->addAccessory(accessory);
        project->setActiveAccessory(accessory);
        std::unique_ptr<AccessoryOpacityValueState> state(new AccessoryOpacityValueState(accessory));
        state->setValue(glm::value_ptr(Vector4(0.7f)));
        SECTION("commited and change opacity")
        {
            state->commit();
            CHECK_THAT(accessory->translation(), Equals(Vector3(1, 2, 3)));
            CHECK_THAT(accessory->orientation(), EqualsRadians(Vector3(18, 45, 73)));
            REQUIRE(accessory->opacity() == Approx(0.7f));
            REQUIRE(accessory->scaleFactor() == Approx(0.9f));
        }
        SECTION("undo and rolled back opacity only")
        {
            state->commit();
            project->handleUndoAction();
            CHECK_THAT(accessory->translation(), Equals(Vector3(1, 2, 3)));
            CHECK_THAT(accessory->orientation(), EqualsRadians(Vector3(18, 45, 73)));
            REQUIRE(accessory->opacity() == Approx(0.4f));
            REQUIRE(accessory->scaleFactor() == Approx(0.9f));
        }
        SECTION("redo and commit again")
        {
            state->commit();
            project->handleUndoAction();
            project->handleRedoAction();
            CHECK_THAT(accessory->translation(), Equals(Vector3(1, 2, 3)));
            CHECK_THAT(accessory->orientation(), EqualsRadians(Vector3(18, 45, 73)));
            REQUIRE(accessory->opacity() == Approx(0.7f));
            REQUIRE(accessory->scaleFactor() == Approx(0.9f));
        }
    }
}
