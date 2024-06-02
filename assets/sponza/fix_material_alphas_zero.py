import bpy

def select_all():
    for o in bpy.context.selected_objects:
        for slot in o.material_slots:
            slot.material.blend_method = "BLEND"
            slot.material.use_backface_culling = True
            slot.material.show_transparent_back = True
            slot.material.node_tree.nodes["Principled BSDF"].inputs[4].default_value = 1.0

select_all()
