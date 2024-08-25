#[derive(PartialEq, PartialOrd)]
pub enum Kind {
    Unused,
    Error,
    Design_File,
    Design_Unit,
    Library_Clause,
    Use_Clause,
    Context_Reference,
    PSL_Inherit_Spec,
    Integer_Literal,
    Floating_Point_Literal,
    Null_Literal,
    String_Literal8,
    Physical_Int_Literal,
    Physical_Fp_Literal,
    Simple_Aggregate,
    Overflow_Literal,
    Unaffected_Waveform,
    Waveform_Element,
    Conditional_Waveform,
    Conditional_Expression,
    Association_Element_By_Expression,
    Association_Element_By_Name,
    Association_Element_By_Individual,
    Association_Element_Open,
    Association_Element_Package,
    Association_Element_Type,
    Association_Element_Subprogram,
    Association_Element_Terminal,
    Choice_By_Range,
    Choice_By_Expression,
    Choice_By_Others,
    Choice_By_None,
    Choice_By_Name,
    Entity_Aspect_Entity,
    Entity_Aspect_Configuration,
    Entity_Aspect_Open,
    Psl_Hierarchical_Name,
    Block_Configuration,
    Block_Header,
    Component_Configuration,
    Binding_Indication,
    Entity_Class,
    Attribute_Value,
    Signature,
    Aggregate_Info,
    Procedure_Call,
    Record_Element_Constraint,
    Array_Element_Resolution,
    Record_Resolution,
    Record_Element_Resolution,
    Simple_Mode_View_Element,
    Array_Mode_View_Element,
    Record_Mode_View_Element,
    Break_Element,
    Attribute_Specification,
    Disconnection_Specification,
    Step_Limit_Specification,
    Configuration_Specification,
    Access_Type_Definition,
    Incomplete_Type_Definition,
    Interface_Type_Definition,
    File_Type_Definition,
    Protected_Type_Declaration,
    Record_Type_Definition,
    Array_Type_Definition,
    Array_Subtype_Definition,
    Record_Subtype_Definition,
    Access_Subtype_Definition,
    File_Subtype_Definition,
    Physical_Subtype_Definition,
    Floating_Subtype_Definition,
    Integer_Subtype_Definition,
    Enumeration_Subtype_Definition,
    Enumeration_Type_Definition,
    Integer_Type_Definition,
    Floating_Type_Definition,
    Physical_Type_Definition,
    Range_Expression,
    Protected_Type_Body,
    Wildcard_Type_Definition,
    Foreign_Vector_Type_Definition,
    Subtype_Definition,
    Record_Mode_View_Indication,
    Array_Mode_View_Indication,
    Scalar_Nature_Definition,
    Record_Nature_Definition,
    Array_Nature_Definition,
    Array_Subnature_Definition,
    Overload_List,
    Foreign_Module,
    Entity_Declaration,
    Configuration_Declaration,
    Context_Declaration,
    Package_Declaration,
    Package_Instantiation_Declaration,
    Vmode_Declaration,
    Vprop_Declaration,
    Vunit_Declaration,
    Package_Body,
    Architecture_Body,
    Package_Instantiation_Body,
    Type_Declaration,
    Anonymous_Type_Declaration,
    Subtype_Declaration,
    Nature_Declaration,
    Subnature_Declaration,
    Package_Header,
    Unit_Declaration,
    Library_Declaration,
    Component_Declaration,
    Attribute_Declaration,
    Group_Template_Declaration,
    Group_Declaration,
    Element_Declaration,
    Nature_Element_Declaration,
    Non_Object_Alias_Declaration,
    Mode_View_Declaration,
    Psl_Declaration,
    Psl_Boolean_Parameter,
    Psl_Endpoint_Declaration,
    Enumeration_Literal,
    Function_Declaration,
    Procedure_Declaration,
    Function_Body,
    Procedure_Body,
    Function_Instantiation_Declaration,
    Procedure_Instantiation_Declaration,
    Terminal_Declaration,
    Object_Alias_Declaration,
    Free_Quantity_Declaration,
    Spectrum_Quantity_Declaration,
    Noise_Quantity_Declaration,
    Across_Quantity_Declaration,
    Through_Quantity_Declaration,
    File_Declaration,
    Guard_Signal_Declaration,
    Signal_Declaration,
    Variable_Declaration,
    Constant_Declaration,
    Iterator_Declaration,
    Interface_Constant_Declaration,
    Interface_Variable_Declaration,
    Interface_Signal_Declaration,
    Interface_View_Declaration,
    Interface_File_Declaration,
    Interface_Quantity_Declaration,
    Interface_Terminal_Declaration,
    Interface_Type_Declaration,
    Interface_Package_Declaration,
    Interface_Function_Declaration,
    Interface_Procedure_Declaration,
    Attribute_Implicit_Declaration,
    Suspend_State_Declaration,
    Identity_Operator,
    Negation_Operator,
    Absolute_Operator,
    Not_Operator,
    Implicit_Condition_Operator,
    Condition_Operator,
    Reduction_And_Operator,
    Reduction_Or_Operator,
    Reduction_Nand_Operator,
    Reduction_Nor_Operator,
    Reduction_Xor_Operator,
    Reduction_Xnor_Operator,
    And_Operator,
    Or_Operator,
    Nand_Operator,
    Nor_Operator,
    Xor_Operator,
    Xnor_Operator,
    Equality_Operator,
    Inequality_Operator,
    Less_Than_Operator,
    Less_Than_Or_Equal_Operator,
    Greater_Than_Operator,
    Greater_Than_Or_Equal_Operator,
    Match_Equality_Operator,
    Match_Inequality_Operator,
    Match_Less_Than_Operator,
    Match_Less_Than_Or_Equal_Operator,
    Match_Greater_Than_Operator,
    Match_Greater_Than_Or_Equal_Operator,
    Sll_Operator,
    Sla_Operator,
    Srl_Operator,
    Sra_Operator,
    Rol_Operator,
    Ror_Operator,
    Addition_Operator,
    Substraction_Operator,
    Concatenation_Operator,
    Multiplication_Operator,
    Division_Operator,
    Modulus_Operator,
    Remainder_Operator,
    Exponentiation_Operator,
    Function_Call,
    Aggregate,
    Parenthesis_Expression,
    Qualified_Expression,
    Type_Conversion,
    Allocator_By_Expression,
    Allocator_By_Subtype,
    Selected_Element,
    Dereference,
    Implicit_Dereference,
    Slice_Name,
    Indexed_Name,
    Psl_Prev,
    Psl_Stable,
    Psl_Rose,
    Psl_Fell,
    Psl_Onehot,
    Psl_Onehot0,
    Psl_Expression,
    Sensitized_Process_Statement,
    Process_Statement,
    Concurrent_Simple_Signal_Assignment,
    Concurrent_Conditional_Signal_Assignment,
    Concurrent_Selected_Signal_Assignment,
    Concurrent_Assertion_Statement,
    Concurrent_Procedure_Call_Statement,
    Concurrent_Break_Statement,
    Psl_Assert_Directive,
    Psl_Assume_Directive,
    Psl_Cover_Directive,
    Psl_Restrict_Directive,
    Block_Statement,
    If_Generate_Statement,
    Case_Generate_Statement,
    For_Generate_Statement,
    Component_Instantiation_Statement,
    Psl_Default_Clock,
    Generate_Statement_Body,
    If_Generate_Else_Clause,
    Simple_Simultaneous_Statement,
    Simultaneous_Null_Statement,
    Simultaneous_Procedural_Statement,
    Simultaneous_Case_Statement,
    Simultaneous_If_Statement,
    Simultaneous_Elsif,
    Simple_Signal_Assignment_Statement,
    Conditional_Signal_Assignment_Statement,
    Selected_Waveform_Assignment_Statement,
    Signal_Force_Assignment_Statement,
    Signal_Release_Assignment_Statement,
    Variable_Assignment_Statement,
    Conditional_Variable_Assignment_Statement,
    Selected_Variable_Assignment_Statement,
    Null_Statement,
    Assertion_Statement,
    Report_Statement,
    Next_Statement,
    Exit_Statement,
    Return_Statement,
    Procedure_Call_Statement,
    Wait_Statement,
    Break_Statement,
    For_Loop_Statement,
    While_Loop_Statement,
    Case_Statement,
    If_Statement,
    Suspend_State_Statement,
    Elsif,
    Character_Literal,
    Simple_Name,
    Selected_Name,
    Operator_Symbol,
    Reference_Name,
    External_Constant_Name,
    External_Signal_Name,
    External_Variable_Name,
    Selected_By_All_Name,
    Parenthesis_Name,
    Package_Pathname,
    Absolute_Pathname,
    Relative_Pathname,
    Pathname_Element,
    Base_Attribute,
    Subtype_Attribute,
    Element_Attribute,
    Across_Attribute,
    Through_Attribute,
    Nature_Reference_Attribute,
    Left_Type_Attribute,
    Right_Type_Attribute,
    High_Type_Attribute,
    Low_Type_Attribute,
    Ascending_Type_Attribute,
    Image_Attribute,
    Value_Attribute,
    Pos_Attribute,
    Val_Attribute,
    Succ_Attribute,
    Pred_Attribute,
    Leftof_Attribute,
    Rightof_Attribute,
    Signal_Slew_Attribute,
    Quantity_Slew_Attribute,
    Ramp_Attribute,
    Zoh_Attribute,
    Ltf_Attribute,
    Ztf_Attribute,
    Dot_Attribute,
    Integ_Attribute,
    Quantity_Delayed_Attribute,
    Above_Attribute,
    Delayed_Attribute,
    Stable_Attribute,
    Quiet_Attribute,
    Transaction_Attribute,
    Event_Attribute,
    Active_Attribute,
    Last_Event_Attribute,
    Last_Active_Attribute,
    Last_Value_Attribute,
    Driving_Attribute,
    Driving_Value_Attribute,
    Behavior_Attribute,
    Structure_Attribute,
    Simple_Name_Attribute,
    Instance_Name_Attribute,
    Path_Name_Attribute,
    Converse_Attribute,
    Left_Array_Attribute,
    Right_Array_Attribute,
    High_Array_Attribute,
    Low_Array_Attribute,
    Length_Array_Attribute,
    Ascending_Array_Attribute,
    Range_Array_Attribute,
    Reverse_Range_Array_Attribute,
    Attribute_Name,
}

impl Kind {
    const VALUES: [Self; 334] = [
        Self::Unused,
        Self::Error,
        Self::Design_File,
        Self::Design_Unit,
        Self::Library_Clause,
        Self::Use_Clause,
        Self::Context_Reference,
        Self::PSL_Inherit_Spec,
        Self::Integer_Literal,
        Self::Floating_Point_Literal,
        Self::Null_Literal,
        Self::String_Literal8,
        Self::Physical_Int_Literal,
        Self::Physical_Fp_Literal,
        Self::Simple_Aggregate,
        Self::Overflow_Literal,
        Self::Unaffected_Waveform,
        Self::Waveform_Element,
        Self::Conditional_Waveform,
        Self::Conditional_Expression,
        Self::Association_Element_By_Expression,
        Self::Association_Element_By_Name,
        Self::Association_Element_By_Individual,
        Self::Association_Element_Open,
        Self::Association_Element_Package,
        Self::Association_Element_Type,
        Self::Association_Element_Subprogram,
        Self::Association_Element_Terminal,
        Self::Choice_By_Range,
        Self::Choice_By_Expression,
        Self::Choice_By_Others,
        Self::Choice_By_None,
        Self::Choice_By_Name,
        Self::Entity_Aspect_Entity,
        Self::Entity_Aspect_Configuration,
        Self::Entity_Aspect_Open,
        Self::Psl_Hierarchical_Name,
        Self::Block_Configuration,
        Self::Block_Header,
        Self::Component_Configuration,
        Self::Binding_Indication,
        Self::Entity_Class,
        Self::Attribute_Value,
        Self::Signature,
        Self::Aggregate_Info,
        Self::Procedure_Call,
        Self::Record_Element_Constraint,
        Self::Array_Element_Resolution,
        Self::Record_Resolution,
        Self::Record_Element_Resolution,
        Self::Simple_Mode_View_Element,
        Self::Array_Mode_View_Element,
        Self::Record_Mode_View_Element,
        Self::Break_Element,
        Self::Attribute_Specification,
        Self::Disconnection_Specification,
        Self::Step_Limit_Specification,
        Self::Configuration_Specification,
        Self::Access_Type_Definition,
        Self::Incomplete_Type_Definition,
        Self::Interface_Type_Definition,
        Self::File_Type_Definition,
        Self::Protected_Type_Declaration,
        Self::Record_Type_Definition,
        Self::Array_Type_Definition,
        Self::Array_Subtype_Definition,
        Self::Record_Subtype_Definition,
        Self::Access_Subtype_Definition,
        Self::File_Subtype_Definition,
        Self::Physical_Subtype_Definition,
        Self::Floating_Subtype_Definition,
        Self::Integer_Subtype_Definition,
        Self::Enumeration_Subtype_Definition,
        Self::Enumeration_Type_Definition,
        Self::Integer_Type_Definition,
        Self::Floating_Type_Definition,
        Self::Physical_Type_Definition,
        Self::Range_Expression,
        Self::Protected_Type_Body,
        Self::Wildcard_Type_Definition,
        Self::Foreign_Vector_Type_Definition,
        Self::Subtype_Definition,
        Self::Record_Mode_View_Indication,
        Self::Array_Mode_View_Indication,
        Self::Scalar_Nature_Definition,
        Self::Record_Nature_Definition,
        Self::Array_Nature_Definition,
        Self::Array_Subnature_Definition,
        Self::Overload_List,
        Self::Foreign_Module,
        Self::Entity_Declaration,
        Self::Configuration_Declaration,
        Self::Context_Declaration,
        Self::Package_Declaration,
        Self::Package_Instantiation_Declaration,
        Self::Vmode_Declaration,
        Self::Vprop_Declaration,
        Self::Vunit_Declaration,
        Self::Package_Body,
        Self::Architecture_Body,
        Self::Package_Instantiation_Body,
        Self::Type_Declaration,
        Self::Anonymous_Type_Declaration,
        Self::Subtype_Declaration,
        Self::Nature_Declaration,
        Self::Subnature_Declaration,
        Self::Package_Header,
        Self::Unit_Declaration,
        Self::Library_Declaration,
        Self::Component_Declaration,
        Self::Attribute_Declaration,
        Self::Group_Template_Declaration,
        Self::Group_Declaration,
        Self::Element_Declaration,
        Self::Nature_Element_Declaration,
        Self::Non_Object_Alias_Declaration,
        Self::Mode_View_Declaration,
        Self::Psl_Declaration,
        Self::Psl_Boolean_Parameter,
        Self::Psl_Endpoint_Declaration,
        Self::Enumeration_Literal,
        Self::Function_Declaration,
        Self::Procedure_Declaration,
        Self::Function_Body,
        Self::Procedure_Body,
        Self::Function_Instantiation_Declaration,
        Self::Procedure_Instantiation_Declaration,
        Self::Terminal_Declaration,
        Self::Object_Alias_Declaration,
        Self::Free_Quantity_Declaration,
        Self::Spectrum_Quantity_Declaration,
        Self::Noise_Quantity_Declaration,
        Self::Across_Quantity_Declaration,
        Self::Through_Quantity_Declaration,
        Self::File_Declaration,
        Self::Guard_Signal_Declaration,
        Self::Signal_Declaration,
        Self::Variable_Declaration,
        Self::Constant_Declaration,
        Self::Iterator_Declaration,
        Self::Interface_Constant_Declaration,
        Self::Interface_Variable_Declaration,
        Self::Interface_Signal_Declaration,
        Self::Interface_View_Declaration,
        Self::Interface_File_Declaration,
        Self::Interface_Quantity_Declaration,
        Self::Interface_Terminal_Declaration,
        Self::Interface_Type_Declaration,
        Self::Interface_Package_Declaration,
        Self::Interface_Function_Declaration,
        Self::Interface_Procedure_Declaration,
        Self::Attribute_Implicit_Declaration,
        Self::Suspend_State_Declaration,
        Self::Identity_Operator,
        Self::Negation_Operator,
        Self::Absolute_Operator,
        Self::Not_Operator,
        Self::Implicit_Condition_Operator,
        Self::Condition_Operator,
        Self::Reduction_And_Operator,
        Self::Reduction_Or_Operator,
        Self::Reduction_Nand_Operator,
        Self::Reduction_Nor_Operator,
        Self::Reduction_Xor_Operator,
        Self::Reduction_Xnor_Operator,
        Self::And_Operator,
        Self::Or_Operator,
        Self::Nand_Operator,
        Self::Nor_Operator,
        Self::Xor_Operator,
        Self::Xnor_Operator,
        Self::Equality_Operator,
        Self::Inequality_Operator,
        Self::Less_Than_Operator,
        Self::Less_Than_Or_Equal_Operator,
        Self::Greater_Than_Operator,
        Self::Greater_Than_Or_Equal_Operator,
        Self::Match_Equality_Operator,
        Self::Match_Inequality_Operator,
        Self::Match_Less_Than_Operator,
        Self::Match_Less_Than_Or_Equal_Operator,
        Self::Match_Greater_Than_Operator,
        Self::Match_Greater_Than_Or_Equal_Operator,
        Self::Sll_Operator,
        Self::Sla_Operator,
        Self::Srl_Operator,
        Self::Sra_Operator,
        Self::Rol_Operator,
        Self::Ror_Operator,
        Self::Addition_Operator,
        Self::Substraction_Operator,
        Self::Concatenation_Operator,
        Self::Multiplication_Operator,
        Self::Division_Operator,
        Self::Modulus_Operator,
        Self::Remainder_Operator,
        Self::Exponentiation_Operator,
        Self::Function_Call,
        Self::Aggregate,
        Self::Parenthesis_Expression,
        Self::Qualified_Expression,
        Self::Type_Conversion,
        Self::Allocator_By_Expression,
        Self::Allocator_By_Subtype,
        Self::Selected_Element,
        Self::Dereference,
        Self::Implicit_Dereference,
        Self::Slice_Name,
        Self::Indexed_Name,
        Self::Psl_Prev,
        Self::Psl_Stable,
        Self::Psl_Rose,
        Self::Psl_Fell,
        Self::Psl_Onehot,
        Self::Psl_Onehot0,
        Self::Psl_Expression,
        Self::Sensitized_Process_Statement,
        Self::Process_Statement,
        Self::Concurrent_Simple_Signal_Assignment,
        Self::Concurrent_Conditional_Signal_Assignment,
        Self::Concurrent_Selected_Signal_Assignment,
        Self::Concurrent_Assertion_Statement,
        Self::Concurrent_Procedure_Call_Statement,
        Self::Concurrent_Break_Statement,
        Self::Psl_Assert_Directive,
        Self::Psl_Assume_Directive,
        Self::Psl_Cover_Directive,
        Self::Psl_Restrict_Directive,
        Self::Block_Statement,
        Self::If_Generate_Statement,
        Self::Case_Generate_Statement,
        Self::For_Generate_Statement,
        Self::Component_Instantiation_Statement,
        Self::Psl_Default_Clock,
        Self::Generate_Statement_Body,
        Self::If_Generate_Else_Clause,
        Self::Simple_Simultaneous_Statement,
        Self::Simultaneous_Null_Statement,
        Self::Simultaneous_Procedural_Statement,
        Self::Simultaneous_Case_Statement,
        Self::Simultaneous_If_Statement,
        Self::Simultaneous_Elsif,
        Self::Simple_Signal_Assignment_Statement,
        Self::Conditional_Signal_Assignment_Statement,
        Self::Selected_Waveform_Assignment_Statement,
        Self::Signal_Force_Assignment_Statement,
        Self::Signal_Release_Assignment_Statement,
        Self::Variable_Assignment_Statement,
        Self::Conditional_Variable_Assignment_Statement,
        Self::Selected_Variable_Assignment_Statement,
        Self::Null_Statement,
        Self::Assertion_Statement,
        Self::Report_Statement,
        Self::Next_Statement,
        Self::Exit_Statement,
        Self::Return_Statement,
        Self::Procedure_Call_Statement,
        Self::Wait_Statement,
        Self::Break_Statement,
        Self::For_Loop_Statement,
        Self::While_Loop_Statement,
        Self::Case_Statement,
        Self::If_Statement,
        Self::Suspend_State_Statement,
        Self::Elsif,
        Self::Character_Literal,
        Self::Simple_Name,
        Self::Selected_Name,
        Self::Operator_Symbol,
        Self::Reference_Name,
        Self::External_Constant_Name,
        Self::External_Signal_Name,
        Self::External_Variable_Name,
        Self::Selected_By_All_Name,
        Self::Parenthesis_Name,
        Self::Package_Pathname,
        Self::Absolute_Pathname,
        Self::Relative_Pathname,
        Self::Pathname_Element,
        Self::Base_Attribute,
        Self::Subtype_Attribute,
        Self::Element_Attribute,
        Self::Across_Attribute,
        Self::Through_Attribute,
        Self::Nature_Reference_Attribute,
        Self::Left_Type_Attribute,
        Self::Right_Type_Attribute,
        Self::High_Type_Attribute,
        Self::Low_Type_Attribute,
        Self::Ascending_Type_Attribute,
        Self::Image_Attribute,
        Self::Value_Attribute,
        Self::Pos_Attribute,
        Self::Val_Attribute,
        Self::Succ_Attribute,
        Self::Pred_Attribute,
        Self::Leftof_Attribute,
        Self::Rightof_Attribute,
        Self::Signal_Slew_Attribute,
        Self::Quantity_Slew_Attribute,
        Self::Ramp_Attribute,
        Self::Zoh_Attribute,
        Self::Ltf_Attribute,
        Self::Ztf_Attribute,
        Self::Dot_Attribute,
        Self::Integ_Attribute,
        Self::Quantity_Delayed_Attribute,
        Self::Above_Attribute,
        Self::Delayed_Attribute,
        Self::Stable_Attribute,
        Self::Quiet_Attribute,
        Self::Transaction_Attribute,
        Self::Event_Attribute,
        Self::Active_Attribute,
        Self::Last_Event_Attribute,
        Self::Last_Active_Attribute,
        Self::Last_Value_Attribute,
        Self::Driving_Attribute,
        Self::Driving_Value_Attribute,
        Self::Behavior_Attribute,
        Self::Structure_Attribute,
        Self::Simple_Name_Attribute,
        Self::Instance_Name_Attribute,
        Self::Path_Name_Attribute,
        Self::Converse_Attribute,
        Self::Left_Array_Attribute,
        Self::Right_Array_Attribute,
        Self::High_Array_Attribute,
        Self::Low_Array_Attribute,
        Self::Length_Array_Attribute,
        Self::Ascending_Array_Attribute,
        Self::Range_Array_Attribute,
        Self::Reverse_Range_Array_Attribute,
        Self::Attribute_Name,
    ];

    const IMAGES: [&'static str; 334] = [
        "unused",
        "error",
        "design_file",
        "design_unit",
        "library_clause",
        "use_clause",
        "context_reference",
        "psl_inherit_spec",
        "integer_literal",
        "floating_point_literal",
        "null_literal",
        "string_literal8",
        "physical_int_literal",
        "physical_fp_literal",
        "simple_aggregate",
        "overflow_literal",
        "unaffected_waveform",
        "waveform_element",
        "conditional_waveform",
        "conditional_expression",
        "association_element_by_expression",
        "association_element_by_name",
        "association_element_by_individual",
        "association_element_open",
        "association_element_package",
        "association_element_type",
        "association_element_subprogram",
        "association_element_terminal",
        "choice_by_range",
        "choice_by_expression",
        "choice_by_others",
        "choice_by_none",
        "choice_by_name",
        "entity_aspect_entity",
        "entity_aspect_configuration",
        "entity_aspect_open",
        "psl_hierarchical_name",
        "block_configuration",
        "block_header",
        "component_configuration",
        "binding_indication",
        "entity_class",
        "attribute_value",
        "signature",
        "aggregate_info",
        "procedure_call",
        "record_element_constraint",
        "array_element_resolution",
        "record_resolution",
        "record_element_resolution",
        "simple_mode_view_element",
        "array_mode_view_element",
        "record_mode_view_element",
        "break_element",
        "attribute_specification",
        "disconnection_specification",
        "step_limit_specification",
        "configuration_specification",
        "access_type_definition",
        "incomplete_type_definition",
        "interface_type_definition",
        "file_type_definition",
        "protected_type_declaration",
        "record_type_definition",
        "array_type_definition",
        "array_subtype_definition",
        "record_subtype_definition",
        "access_subtype_definition",
        "file_subtype_definition",
        "physical_subtype_definition",
        "floating_subtype_definition",
        "integer_subtype_definition",
        "enumeration_subtype_definition",
        "enumeration_type_definition",
        "integer_type_definition",
        "floating_type_definition",
        "physical_type_definition",
        "range_expression",
        "protected_type_body",
        "wildcard_type_definition",
        "foreign_vector_type_definition",
        "subtype_definition",
        "record_mode_view_indication",
        "array_mode_view_indication",
        "scalar_nature_definition",
        "record_nature_definition",
        "array_nature_definition",
        "array_subnature_definition",
        "overload_list",
        "foreign_module",
        "entity_declaration",
        "configuration_declaration",
        "context_declaration",
        "package_declaration",
        "package_instantiation_declaration",
        "vmode_declaration",
        "vprop_declaration",
        "vunit_declaration",
        "package_body",
        "architecture_body",
        "package_instantiation_body",
        "type_declaration",
        "anonymous_type_declaration",
        "subtype_declaration",
        "nature_declaration",
        "subnature_declaration",
        "package_header",
        "unit_declaration",
        "library_declaration",
        "component_declaration",
        "attribute_declaration",
        "group_template_declaration",
        "group_declaration",
        "element_declaration",
        "nature_element_declaration",
        "non_object_alias_declaration",
        "mode_view_declaration",
        "psl_declaration",
        "psl_boolean_parameter",
        "psl_endpoint_declaration",
        "enumeration_literal",
        "function_declaration",
        "procedure_declaration",
        "function_body",
        "procedure_body",
        "function_instantiation_declaration",
        "procedure_instantiation_declaration",
        "terminal_declaration",
        "object_alias_declaration",
        "free_quantity_declaration",
        "spectrum_quantity_declaration",
        "noise_quantity_declaration",
        "across_quantity_declaration",
        "through_quantity_declaration",
        "file_declaration",
        "guard_signal_declaration",
        "signal_declaration",
        "variable_declaration",
        "constant_declaration",
        "iterator_declaration",
        "interface_constant_declaration",
        "interface_variable_declaration",
        "interface_signal_declaration",
        "interface_view_declaration",
        "interface_file_declaration",
        "interface_quantity_declaration",
        "interface_terminal_declaration",
        "interface_type_declaration",
        "interface_package_declaration",
        "interface_function_declaration",
        "interface_procedure_declaration",
        "attribute_implicit_declaration",
        "suspend_state_declaration",
        "identity_operator",
        "negation_operator",
        "absolute_operator",
        "not_operator",
        "implicit_condition_operator",
        "condition_operator",
        "reduction_and_operator",
        "reduction_or_operator",
        "reduction_nand_operator",
        "reduction_nor_operator",
        "reduction_xor_operator",
        "reduction_xnor_operator",
        "and_operator",
        "or_operator",
        "nand_operator",
        "nor_operator",
        "xor_operator",
        "xnor_operator",
        "equality_operator",
        "inequality_operator",
        "less_than_operator",
        "less_than_or_equal_operator",
        "greater_than_operator",
        "greater_than_or_equal_operator",
        "match_equality_operator",
        "match_inequality_operator",
        "match_less_than_operator",
        "match_less_than_or_equal_operator",
        "match_greater_than_operator",
        "match_greater_than_or_equal_operator",
        "sll_operator",
        "sla_operator",
        "srl_operator",
        "sra_operator",
        "rol_operator",
        "ror_operator",
        "addition_operator",
        "substraction_operator",
        "concatenation_operator",
        "multiplication_operator",
        "division_operator",
        "modulus_operator",
        "remainder_operator",
        "exponentiation_operator",
        "function_call",
        "aggregate",
        "parenthesis_expression",
        "qualified_expression",
        "type_conversion",
        "allocator_by_expression",
        "allocator_by_subtype",
        "selected_element",
        "dereference",
        "implicit_dereference",
        "slice_name",
        "indexed_name",
        "psl_prev",
        "psl_stable",
        "psl_rose",
        "psl_fell",
        "psl_onehot",
        "psl_onehot0",
        "psl_expression",
        "sensitized_process_statement",
        "process_statement",
        "concurrent_simple_signal_assignment",
        "concurrent_conditional_signal_assignment",
        "concurrent_selected_signal_assignment",
        "concurrent_assertion_statement",
        "concurrent_procedure_call_statement",
        "concurrent_break_statement",
        "psl_assert_directive",
        "psl_assume_directive",
        "psl_cover_directive",
        "psl_restrict_directive",
        "block_statement",
        "if_generate_statement",
        "case_generate_statement",
        "for_generate_statement",
        "component_instantiation_statement",
        "psl_default_clock",
        "generate_statement_body",
        "if_generate_else_clause",
        "simple_simultaneous_statement",
        "simultaneous_null_statement",
        "simultaneous_procedural_statement",
        "simultaneous_case_statement",
        "simultaneous_if_statement",
        "simultaneous_elsif",
        "simple_signal_assignment_statement",
        "conditional_signal_assignment_statement",
        "selected_waveform_assignment_statement",
        "signal_force_assignment_statement",
        "signal_release_assignment_statement",
        "variable_assignment_statement",
        "conditional_variable_assignment_statement",
        "selected_variable_assignment_statement",
        "null_statement",
        "assertion_statement",
        "report_statement",
        "next_statement",
        "exit_statement",
        "return_statement",
        "procedure_call_statement",
        "wait_statement",
        "break_statement",
        "for_loop_statement",
        "while_loop_statement",
        "case_statement",
        "if_statement",
        "suspend_state_statement",
        "elsif",
        "character_literal",
        "simple_name",
        "selected_name",
        "operator_symbol",
        "reference_name",
        "external_constant_name",
        "external_signal_name",
        "external_variable_name",
        "selected_by_all_name",
        "parenthesis_name",
        "package_pathname",
        "absolute_pathname",
        "relative_pathname",
        "pathname_element",
        "base_attribute",
        "subtype_attribute",
        "element_attribute",
        "across_attribute",
        "through_attribute",
        "nature_reference_attribute",
        "left_type_attribute",
        "right_type_attribute",
        "high_type_attribute",
        "low_type_attribute",
        "ascending_type_attribute",
        "image_attribute",
        "value_attribute",
        "pos_attribute",
        "val_attribute",
        "succ_attribute",
        "pred_attribute",
        "leftof_attribute",
        "rightof_attribute",
        "signal_slew_attribute",
        "quantity_slew_attribute",
        "ramp_attribute",
        "zoh_attribute",
        "ltf_attribute",
        "ztf_attribute",
        "dot_attribute",
        "integ_attribute",
        "quantity_delayed_attribute",
        "above_attribute",
        "delayed_attribute",
        "stable_attribute",
        "quiet_attribute",
        "transaction_attribute",
        "event_attribute",
        "active_attribute",
        "last_event_attribute",
        "last_active_attribute",
        "last_value_attribute",
        "driving_attribute",
        "driving_value_attribute",
        "behavior_attribute",
        "structure_attribute",
        "simple_name_attribute",
        "instance_name_attribute",
        "path_name_attribute",
        "converse_attribute",
        "left_array_attribute",
        "right_array_attribute",
        "high_array_attribute",
        "low_array_attribute",
        "length_array_attribute",
        "ascending_array_attribute",
        "range_array_attribute",
        "reverse_range_array_attribute",
        "attribute_name",
    ];
}
impl Kind {
    fn is_library_unit(self: Self) -> bool {
        self >= Kind::Foreign_Module && self <= Kind::Architecture_Body
    }

    fn is_primary_unit(self: Self) -> bool {
        self >= Kind::Entity_Declaration && self <= Kind::Vunit_Declaration
    }

    fn is_secondary_unit(self: Self) -> bool {
        self >= Kind::Package_Body && self <= Kind::Architecture_Body
    }

    fn is_package_declaration(self: Self) -> bool {
        self >= Kind::Package_Declaration && self <= Kind::Package_Instantiation_Declaration
    }

    fn is_verification_unit(self: Self) -> bool {
        self >= Kind::Vmode_Declaration && self <= Kind::Vunit_Declaration
    }

    fn is_literal(self: Self) -> bool {
        self >= Kind::Integer_Literal && self <= Kind::Physical_Fp_Literal
    }

    fn is_physical_literal(self: Self) -> bool {
        self >= Kind::Physical_Int_Literal && self <= Kind::Physical_Fp_Literal
    }

    fn is_array_type_definition(self: Self) -> bool {
        self >= Kind::Array_Type_Definition && self <= Kind::Array_Subtype_Definition
    }

    fn is_type_and_subtype_definition(self: Self) -> bool {
        self >= Kind::Access_Type_Definition && self <= Kind::Physical_Type_Definition
    }

    fn is_subtype_definition(self: Self) -> bool {
        self >= Kind::Array_Subtype_Definition && self <= Kind::Enumeration_Subtype_Definition
    }

    fn is_scalar_subtype_definition(self: Self) -> bool {
        self >= Kind::Physical_Subtype_Definition && self <= Kind::Enumeration_Subtype_Definition
    }

    fn is_scalar_type_and_subtype_definition(self: Self) -> bool {
        self >= Kind::Physical_Subtype_Definition && self <= Kind::Physical_Type_Definition
    }

    fn is_range_type_definition(self: Self) -> bool {
        self >= Kind::Physical_Subtype_Definition && self <= Kind::Enumeration_Type_Definition
    }

    fn is_discrete_type_definition(self: Self) -> bool {
        self >= Kind::Integer_Subtype_Definition && self <= Kind::Integer_Type_Definition
    }

    fn is_composite_type_definition(self: Self) -> bool {
        self >= Kind::Record_Type_Definition && self <= Kind::Record_Subtype_Definition
    }

    fn is_composite_subtype_definition(self: Self) -> bool {
        self >= Kind::Array_Subtype_Definition && self <= Kind::Record_Subtype_Definition
    }

    fn is_type_declaration(self: Self) -> bool {
        self >= Kind::Type_Declaration && self <= Kind::Subtype_Declaration
    }

    fn is_nature_definition(self: Self) -> bool {
        self >= Kind::Scalar_Nature_Definition && self <= Kind::Array_Nature_Definition
    }

    fn is_subnature_definition(self: Self) -> bool {
        self >= Kind::Array_Subnature_Definition && self <= Kind::Array_Subnature_Definition
    }

    fn is_nature_indication(self: Self) -> bool {
        self >= Kind::Scalar_Nature_Definition && self <= Kind::Array_Subnature_Definition
    }

    fn is_nonoverloadable_declaration(self: Self) -> bool {
        self >= Kind::Type_Declaration && self <= Kind::Nature_Element_Declaration
    }

    fn is_monadic_operator(self: Self) -> bool {
        self >= Kind::Identity_Operator && self <= Kind::Reduction_Xnor_Operator
    }

    fn is_dyadic_operator(self: Self) -> bool {
        self >= Kind::And_Operator && self <= Kind::Exponentiation_Operator
    }

    fn is_psl_builtin(self: Self) -> bool {
        self >= Kind::Psl_Prev && self <= Kind::Psl_Onehot0
    }

    fn is_functions_and_literals(self: Self) -> bool {
        self >= Kind::Enumeration_Literal && self <= Kind::Function_Declaration
    }

    fn is_subprogram_declaration(self: Self) -> bool {
        self >= Kind::Function_Declaration && self <= Kind::Procedure_Declaration
    }

    fn is_subprogram_body(self: Self) -> bool {
        self >= Kind::Function_Body && self <= Kind::Procedure_Body
    }

    fn is_subprogram_instantiation_declaration(self: Self) -> bool {
        self >= Kind::Function_Instantiation_Declaration && self <= Kind::Procedure_Instantiation_Declaration
    }

    fn is_interface_object_declaration(self: Self) -> bool {
        self >= Kind::Interface_Constant_Declaration && self <= Kind::Interface_Quantity_Declaration
    }

    fn is_interface_subprogram_declaration(self: Self) -> bool {
        self >= Kind::Interface_Function_Declaration && self <= Kind::Interface_Procedure_Declaration
    }

    fn is_interface_declaration(self: Self) -> bool {
        self >= Kind::Interface_Constant_Declaration && self <= Kind::Interface_Procedure_Declaration
    }

    fn is_object_declaration(self: Self) -> bool {
        self >= Kind::Object_Alias_Declaration && self <= Kind::Interface_Quantity_Declaration
    }

    fn is_branch_quantity_declaration(self: Self) -> bool {
        self >= Kind::Across_Quantity_Declaration && self <= Kind::Through_Quantity_Declaration
    }

    fn is_source_quantity_declaration(self: Self) -> bool {
        self >= Kind::Spectrum_Quantity_Declaration && self <= Kind::Noise_Quantity_Declaration
    }

    fn is_quantity_declaration(self: Self) -> bool {
        self >= Kind::Free_Quantity_Declaration && self <= Kind::Through_Quantity_Declaration
    }

    fn is_non_alias_object_declaration(self: Self) -> bool {
        self >= Kind::File_Declaration && self <= Kind::Interface_File_Declaration
    }

    fn is_association_element_parameters(self: Self) -> bool {
        self >= Kind::Association_Element_By_Expression && self <= Kind::Association_Element_Open
    }

    fn is_association_element_by_actual(self: Self) -> bool {
        self >= Kind::Association_Element_By_Expression && self <= Kind::Association_Element_By_Name
    }

    fn is_association_element(self: Self) -> bool {
        self >= Kind::Association_Element_By_Expression && self <= Kind::Association_Element_Terminal
    }

    fn is_choice(self: Self) -> bool {
        self >= Kind::Choice_By_Range && self <= Kind::Choice_By_Name
    }

    fn is_case_choice(self: Self) -> bool {
        self >= Kind::Choice_By_Range && self <= Kind::Choice_By_Others
    }

    fn is_array_choice(self: Self) -> bool {
        self >= Kind::Choice_By_Range && self <= Kind::Choice_By_None
    }

    fn is_record_choice(self: Self) -> bool {
        self >= Kind::Choice_By_Others && self <= Kind::Choice_By_Name
    }

    fn is_entity_aspect(self: Self) -> bool {
        self >= Kind::Entity_Aspect_Entity && self <= Kind::Entity_Aspect_Open
    }

    fn is_denoting_name(self: Self) -> bool {
        self >= Kind::Character_Literal && self <= Kind::Reference_Name
    }

    fn is_denoting_and_external_name(self: Self) -> bool {
        self >= Kind::Character_Literal && self <= Kind::External_Variable_Name
    }

    fn is_name(self: Self) -> bool {
        self >= Kind::Character_Literal && self <= Kind::Parenthesis_Name
    }

    fn is_dereference(self: Self) -> bool {
        self >= Kind::Dereference && self <= Kind::Implicit_Dereference
    }

    fn is_external_name(self: Self) -> bool {
        self >= Kind::External_Constant_Name && self <= Kind::External_Variable_Name
    }

    fn is_pathname(self: Self) -> bool {
        self >= Kind::Package_Pathname && self <= Kind::Pathname_Element
    }

    fn is_expression_attribute(self: Self) -> bool {
        self >= Kind::Left_Type_Attribute && self <= Kind::Ascending_Array_Attribute
    }

    fn is_attribute(self: Self) -> bool {
        self >= Kind::Base_Attribute && self <= Kind::Reverse_Range_Array_Attribute
    }

    fn is_type_attribute(self: Self) -> bool {
        self >= Kind::Left_Type_Attribute && self <= Kind::Ascending_Type_Attribute
    }

    fn is_subtype_attribute(self: Self) -> bool {
        self >= Kind::Base_Attribute && self <= Kind::Element_Attribute
    }

    fn is_scalar_type_attribute(self: Self) -> bool {
        self >= Kind::Pos_Attribute && self <= Kind::Rightof_Attribute
    }

    fn is_array_attribute(self: Self) -> bool {
        self >= Kind::Left_Array_Attribute && self <= Kind::Reverse_Range_Array_Attribute
    }

    fn is_range_attribute(self: Self) -> bool {
        self >= Kind::Range_Array_Attribute && self <= Kind::Reverse_Range_Array_Attribute
    }

    fn is_signal_attribute(self: Self) -> bool {
        self >= Kind::Delayed_Attribute && self <= Kind::Transaction_Attribute
    }

    fn is_ams_signal_attribute(self: Self) -> bool {
        self >= Kind::Above_Attribute && self <= Kind::Transaction_Attribute
    }

    fn is_signal_value_attribute(self: Self) -> bool {
        self >= Kind::Event_Attribute && self <= Kind::Driving_Value_Attribute
    }

    fn is_name_attribute(self: Self) -> bool {
        self >= Kind::Simple_Name_Attribute && self <= Kind::Path_Name_Attribute
    }

    fn is_concurrent_statement(self: Self) -> bool {
        self >= Kind::Sensitized_Process_Statement && self <= Kind::Component_Instantiation_Statement
    }

    fn is_structural_statement(self: Self) -> bool {
        self >= Kind::Block_Statement && self <= Kind::Component_Instantiation_Statement
    }

    fn is_simple_concurrent_statement(self: Self) -> bool {
        self >= Kind::Sensitized_Process_Statement && self <= Kind::Psl_Restrict_Directive
    }

    fn is_process_statement(self: Self) -> bool {
        self >= Kind::Sensitized_Process_Statement && self <= Kind::Process_Statement
    }

    fn is_concurrent_signal_assignment(self: Self) -> bool {
        self >= Kind::Concurrent_Simple_Signal_Assignment && self <= Kind::Concurrent_Selected_Signal_Assignment
    }

    fn is_psl_property_directive(self: Self) -> bool {
        self >= Kind::Psl_Assert_Directive && self <= Kind::Psl_Assume_Directive
    }

    fn is_psl_sequence_directive(self: Self) -> bool {
        self >= Kind::Psl_Cover_Directive && self <= Kind::Psl_Restrict_Directive
    }

    fn is_psl_directive(self: Self) -> bool {
        self >= Kind::Psl_Assert_Directive && self <= Kind::Psl_Restrict_Directive
    }

    fn is_generate_statement(self: Self) -> bool {
        self >= Kind::If_Generate_Statement && self <= Kind::For_Generate_Statement
    }

    fn is_if_case_generate_statement(self: Self) -> bool {
        self >= Kind::If_Generate_Statement && self <= Kind::Case_Generate_Statement
    }

    fn is_simultaneous_statement(self: Self) -> bool {
        self >= Kind::Simple_Simultaneous_Statement && self <= Kind::Simultaneous_If_Statement
    }

    fn is_sequential_statement(self: Self) -> bool {
        self >= Kind::Simple_Signal_Assignment_Statement && self <= Kind::If_Statement
    }

    fn is_signal_assignment_statement(self: Self) -> bool {
        self >= Kind::Simple_Signal_Assignment_Statement && self <= Kind::Signal_Release_Assignment_Statement
    }

    fn is_sequential_statement_ext(self: Self) -> bool {
        self >= Kind::Simple_Signal_Assignment_Statement && self <= Kind::Suspend_State_Statement
    }

    fn is_next_exit_statement(self: Self) -> bool {
        self >= Kind::Next_Statement && self <= Kind::Exit_Statement
    }

    fn is_variable_assignment_statement(self: Self) -> bool {
        self >= Kind::Variable_Assignment_Statement && self <= Kind::Selected_Variable_Assignment_Statement
    }

    fn is_allocator(self: Self) -> bool {
        self >= Kind::Allocator_By_Expression && self <= Kind::Allocator_By_Subtype
    }

    fn is_clause(self: Self) -> bool {
        self >= Kind::Library_Clause && self <= Kind::Context_Reference
    }

    fn is_specification(self: Self) -> bool {
        self >= Kind::Attribute_Specification && self <= Kind::Configuration_Specification
    }

    fn is_mode_view_element_definition(self: Self) -> bool {
        self >= Kind::Simple_Mode_View_Element && self <= Kind::Record_Mode_View_Element
    }

}
pub enum Iir_Mode {
    Unknown_Mode,
    Linkage_Mode,
    Buffer_Mode,
    Out_Mode,
    Inout_Mode,
    In_Mode,
}

impl Iir_Mode {
    const VALUES: [Self; 6] = [
        Self::Unknown_Mode,
        Self::Linkage_Mode,
        Self::Buffer_Mode,
        Self::Out_Mode,
        Self::Inout_Mode,
        Self::In_Mode,
    ];

    const IMAGES: [&'static str; 6] = [
        "unknown_mode",
        "linkage_mode",
        "buffer_mode",
        "out_mode",
        "inout_mode",
        "in_mode",
    ];
}
pub enum ScalarSize {
    Scalar_8,
    Scalar_16,
    Scalar_32,
    Scalar_64,
}

impl ScalarSize {
    const VALUES: [Self; 4] = [
        Self::Scalar_8,
        Self::Scalar_16,
        Self::Scalar_32,
        Self::Scalar_64,
    ];

    const IMAGES: [&'static str; 4] = [
        "scalar_8",
        "scalar_16",
        "scalar_32",
        "scalar_64",
    ];
}
pub enum Iir_Staticness {
    Unknown,
    None,
    Globally,
    Locally,
}

impl Iir_Staticness {
    const VALUES: [Self; 4] = [
        Self::Unknown,
        Self::None,
        Self::Globally,
        Self::Locally,
    ];

    const IMAGES: [&'static str; 4] = [
        "unknown",
        "none",
        "globally",
        "locally",
    ];
}
pub enum Iir_Constraint {
    Unconstrained,
    Partially_Constrained,
    Fully_Constrained,
}

impl Iir_Constraint {
    const VALUES: [Self; 3] = [
        Self::Unconstrained,
        Self::Partially_Constrained,
        Self::Fully_Constrained,
    ];

    const IMAGES: [&'static str; 3] = [
        "unconstrained",
        "partially_constrained",
        "fully_constrained",
    ];
}
pub enum Iir_Delay_Mechanism {
    Inertial_Delay,
    Transport_Delay,
}

impl Iir_Delay_Mechanism {
    const VALUES: [Self; 2] = [
        Self::Inertial_Delay,
        Self::Transport_Delay,
    ];

    const IMAGES: [&'static str; 2] = [
        "inertial_delay",
        "transport_delay",
    ];
}
pub enum DateStateType {
    Extern,
    Disk,
    Parse,
    Analyze,
}

impl DateStateType {
    const VALUES: [Self; 4] = [
        Self::Extern,
        Self::Disk,
        Self::Parse,
        Self::Analyze,
    ];

    const IMAGES: [&'static str; 4] = [
        "extern",
        "disk",
        "parse",
        "analyze",
    ];
}
pub enum NumberBaseType {
    Base_None,
    Base_2,
    Base_8,
    Base_10,
    Base_16,
}

impl NumberBaseType {
    const VALUES: [Self; 5] = [
        Self::Base_None,
        Self::Base_2,
        Self::Base_8,
        Self::Base_10,
        Self::Base_16,
    ];

    const IMAGES: [&'static str; 5] = [
        "base_none",
        "base_2",
        "base_8",
        "base_10",
        "base_16",
    ];
}
pub enum Iir_Predefined {
    Error,
    Boolean_And,
    Boolean_Or,
    Boolean_Nand,
    Boolean_Nor,
    Boolean_Xor,
    Boolean_Xnor,
    Boolean_Not,
    Boolean_Rising_Edge,
    Boolean_Falling_Edge,
    Enum_Equality,
    Enum_Inequality,
    Enum_Less,
    Enum_Less_Equal,
    Enum_Greater,
    Enum_Greater_Equal,
    Bit_And,
    Bit_Or,
    Bit_Nand,
    Bit_Nor,
    Bit_Xor,
    Bit_Xnor,
    Bit_Not,
    Bit_Match_Equality,
    Bit_Match_Inequality,
    Bit_Match_Less,
    Bit_Match_Less_Equal,
    Bit_Match_Greater,
    Bit_Match_Greater_Equal,
    Bit_Condition,
    Integer_Equality,
    Integer_Inequality,
    Integer_Less,
    Integer_Less_Equal,
    Integer_Greater,
    Integer_Greater_Equal,
    Integer_Identity,
    Integer_Negation,
    Integer_Absolute,
    Integer_Plus,
    Integer_Minus,
    Integer_Mul,
    Integer_Div,
    Integer_Mod,
    Integer_Rem,
    Integer_Exp,
    Floating_Equality,
    Floating_Inequality,
    Floating_Less,
    Floating_Less_Equal,
    Floating_Greater,
    Floating_Greater_Equal,
    Floating_Identity,
    Floating_Negation,
    Floating_Absolute,
    Floating_Plus,
    Floating_Minus,
    Floating_Mul,
    Floating_Div,
    Floating_Exp,
    Universal_R_I_Mul,
    Universal_I_R_Mul,
    Universal_R_I_Div,
    Physical_Equality,
    Physical_Inequality,
    Physical_Less,
    Physical_Less_Equal,
    Physical_Greater,
    Physical_Greater_Equal,
    Physical_Identity,
    Physical_Negation,
    Physical_Absolute,
    Physical_Plus,
    Physical_Minus,
    Physical_Integer_Mul,
    Physical_Real_Mul,
    Integer_Physical_Mul,
    Real_Physical_Mul,
    Physical_Integer_Div,
    Physical_Real_Div,
    Physical_Physical_Div,
    Physical_Mod,
    Physical_Rem,
    Access_Equality,
    Access_Inequality,
    Record_Equality,
    Record_Inequality,
    Array_Equality,
    Array_Inequality,
    Array_Less,
    Array_Less_Equal,
    Array_Greater,
    Array_Greater_Equal,
    Array_Array_Concat,
    Array_Element_Concat,
    Element_Array_Concat,
    Element_Element_Concat,
    Array_Minimum,
    Array_Maximum,
    Vector_Minimum,
    Vector_Maximum,
    Array_Sll,
    Array_Srl,
    Array_Sla,
    Array_Sra,
    Array_Rol,
    Array_Ror,
    TF_Array_And,
    TF_Array_Or,
    TF_Array_Nand,
    TF_Array_Nor,
    TF_Array_Xor,
    TF_Array_Xnor,
    TF_Array_Not,
    TF_Reduction_And,
    TF_Reduction_Or,
    TF_Reduction_Nand,
    TF_Reduction_Nor,
    TF_Reduction_Xor,
    TF_Reduction_Xnor,
    TF_Reduction_Not,
    TF_Array_Element_And,
    TF_Element_Array_And,
    TF_Array_Element_Or,
    TF_Element_Array_Or,
    TF_Array_Element_Nand,
    TF_Element_Array_Nand,
    TF_Array_Element_Nor,
    TF_Element_Array_Nor,
    TF_Array_Element_Xor,
    TF_Element_Array_Xor,
    TF_Array_Element_Xnor,
    TF_Element_Array_Xnor,
    Bit_Array_Match_Equality,
    Bit_Array_Match_Inequality,
    Std_Ulogic_Match_Equality,
    Std_Ulogic_Match_Inequality,
    Std_Ulogic_Match_Less,
    Std_Ulogic_Match_Less_Equal,
    Std_Ulogic_Match_Greater,
    Std_Ulogic_Match_Greater_Equal,
    Std_Ulogic_Array_Match_Equality,
    Std_Ulogic_Array_Match_Inequality,
    Enum_Minimum,
    Enum_Maximum,
    Enum_To_String,
    Integer_Minimum,
    Integer_Maximum,
    Integer_To_String,
    Bit_Rising_Edge,
    Bit_Falling_Edge,
    Floating_Minimum,
    Floating_Maximum,
    Floating_To_String,
    Real_To_String_Digits,
    Real_To_String_Format,
    Physical_Minimum,
    Physical_Maximum,
    Physical_To_String,
    Time_To_String_Unit,
    Array_Char_To_String,
    Bit_Vector_To_Ostring,
    Bit_Vector_To_Hstring,
    Deallocate,
    File_Open,
    File_Open_Status,
    File_Close,
    Read,
    Read_Length,
    Flush,
    Write,
    Endfile,
    Now_Function,
    Real_Now_Function,
    Frequency_Function,
    None,
    Foreign_Untruncated_Text_Read,
    Foreign_Textio_Read_Real,
    Foreign_Textio_Write_Real,
    Std_Env_Stop_Status,
    Std_Env_Stop,
    Std_Env_Finish_Status,
    Std_Env_Finish,
    Std_Env_Resolution_Limit,
    Ieee_1164_Scalar_And,
    Ieee_1164_Scalar_Nand,
    Ieee_1164_Scalar_Or,
    Ieee_1164_Scalar_Nor,
    Ieee_1164_Scalar_Xor,
    Ieee_1164_Scalar_Xnor,
    Ieee_1164_Scalar_Not,
    Ieee_1164_Vector_And,
    Ieee_1164_Vector_Nand,
    Ieee_1164_Vector_Or,
    Ieee_1164_Vector_Nor,
    Ieee_1164_Vector_Xor,
    Ieee_1164_Vector_Xnor,
    Ieee_1164_Vector_Not,
    Ieee_1164_To_Bit,
    Ieee_1164_To_Bitvector,
    Ieee_1164_To_Stdulogic,
    Ieee_1164_To_Stdlogicvector_Bv,
    Ieee_1164_To_Stdlogicvector_Suv,
    Ieee_1164_To_Stdulogicvector_Bv,
    Ieee_1164_To_Stdulogicvector_Slv,
    Ieee_1164_To_X01_Slv,
    Ieee_1164_To_X01_Suv,
    Ieee_1164_To_X01_Log,
    Ieee_1164_To_X01_Bv_Slv,
    Ieee_1164_To_X01_Bv_Suv,
    Ieee_1164_To_X01_Bit_Log,
    Ieee_1164_To_X01Z_Slv,
    Ieee_1164_To_X01Z_Suv,
    Ieee_1164_To_X01Z_Log,
    Ieee_1164_To_X01Z_Bv_Slv,
    Ieee_1164_To_X01Z_Bv_Suv,
    Ieee_1164_To_X01Z_Bit_Log,
    Ieee_1164_To_UX01_Slv,
    Ieee_1164_To_UX01_Suv,
    Ieee_1164_To_UX01_Log,
    Ieee_1164_To_UX01_Bv_Slv,
    Ieee_1164_To_UX01_Bv_Suv,
    Ieee_1164_To_UX01_Bit_Log,
    Ieee_1164_Is_X_Slv,
    Ieee_1164_Is_X_Log,
    Ieee_1164_Rising_Edge,
    Ieee_1164_Falling_Edge,
    Ieee_1164_And_Suv_Log,
    Ieee_1164_And_Log_Suv,
    Ieee_1164_Nand_Suv_Log,
    Ieee_1164_Nand_Log_Suv,
    Ieee_1164_Or_Suv_Log,
    Ieee_1164_Or_Log_Suv,
    Ieee_1164_Nor_Suv_Log,
    Ieee_1164_Nor_Log_Suv,
    Ieee_1164_Xor_Suv_Log,
    Ieee_1164_Xor_Log_Suv,
    Ieee_1164_Xnor_Suv_Log,
    Ieee_1164_Xnor_Log_Suv,
    Ieee_1164_And_Suv,
    Ieee_1164_Nand_Suv,
    Ieee_1164_Or_Suv,
    Ieee_1164_Nor_Suv,
    Ieee_1164_Xor_Suv,
    Ieee_1164_Xnor_Suv,
    Ieee_1164_Vector_Sll,
    Ieee_1164_Vector_Srl,
    Ieee_1164_Vector_Rol,
    Ieee_1164_Vector_Ror,
    Ieee_1164_Condition_Operator,
    Ieee_1164_To_01_Log_Log,
    Ieee_1164_To_01_Slv_Log,
    Ieee_1164_To_Hstring,
    Ieee_1164_To_Ostring,
    Ieee_Numeric_Std_Toint_Uns_Nat,
    Ieee_Numeric_Std_Toint_Sgn_Int,
    Ieee_Numeric_Std_Touns_Nat_Nat_Uns,
    Ieee_Numeric_Std_Touns_Nat_Uns_Uns,
    Ieee_Numeric_Std_Tosgn_Int_Nat_Sgn,
    Ieee_Numeric_Std_Tosgn_Int_Sgn_Sgn,
    Ieee_Numeric_Std_Resize_Uns_Nat,
    Ieee_Numeric_Std_Resize_Sgn_Nat,
    Ieee_Numeric_Std_Resize_Uns_Uns,
    Ieee_Numeric_Std_Resize_Sgn_Sgn,
    Ieee_Numeric_Std_Add_Uns_Uns,
    Ieee_Numeric_Std_Add_Uns_Nat,
    Ieee_Numeric_Std_Add_Nat_Uns,
    Ieee_Numeric_Std_Add_Uns_Log,
    Ieee_Numeric_Std_Add_Log_Uns,
    Ieee_Numeric_Std_Add_Sgn_Sgn,
    Ieee_Numeric_Std_Add_Sgn_Int,
    Ieee_Numeric_Std_Add_Int_Sgn,
    Ieee_Numeric_Std_Add_Sgn_Log,
    Ieee_Numeric_Std_Add_Log_Sgn,
    Ieee_Numeric_Std_Sub_Uns_Uns,
    Ieee_Numeric_Std_Sub_Uns_Nat,
    Ieee_Numeric_Std_Sub_Nat_Uns,
    Ieee_Numeric_Std_Sub_Uns_Log,
    Ieee_Numeric_Std_Sub_Log_Uns,
    Ieee_Numeric_Std_Sub_Sgn_Sgn,
    Ieee_Numeric_Std_Sub_Sgn_Int,
    Ieee_Numeric_Std_Sub_Int_Sgn,
    Ieee_Numeric_Std_Sub_Sgn_Log,
    Ieee_Numeric_Std_Sub_Log_Sgn,
    Ieee_Numeric_Std_Mul_Uns_Uns,
    Ieee_Numeric_Std_Mul_Uns_Nat,
    Ieee_Numeric_Std_Mul_Nat_Uns,
    Ieee_Numeric_Std_Mul_Sgn_Sgn,
    Ieee_Numeric_Std_Mul_Sgn_Int,
    Ieee_Numeric_Std_Mul_Int_Sgn,
    Ieee_Numeric_Std_Div_Uns_Uns,
    Ieee_Numeric_Std_Div_Uns_Nat,
    Ieee_Numeric_Std_Div_Nat_Uns,
    Ieee_Numeric_Std_Div_Sgn_Sgn,
    Ieee_Numeric_Std_Div_Sgn_Int,
    Ieee_Numeric_Std_Div_Int_Sgn,
    Ieee_Numeric_Std_Rem_Uns_Uns,
    Ieee_Numeric_Std_Rem_Uns_Nat,
    Ieee_Numeric_Std_Rem_Nat_Uns,
    Ieee_Numeric_Std_Rem_Sgn_Sgn,
    Ieee_Numeric_Std_Rem_Sgn_Int,
    Ieee_Numeric_Std_Rem_Int_Sgn,
    Ieee_Numeric_Std_Mod_Uns_Uns,
    Ieee_Numeric_Std_Mod_Uns_Nat,
    Ieee_Numeric_Std_Mod_Nat_Uns,
    Ieee_Numeric_Std_Mod_Sgn_Sgn,
    Ieee_Numeric_Std_Mod_Sgn_Int,
    Ieee_Numeric_Std_Mod_Int_Sgn,
    Ieee_Numeric_Std_Gt_Uns_Uns,
    Ieee_Numeric_Std_Gt_Uns_Nat,
    Ieee_Numeric_Std_Gt_Nat_Uns,
    Ieee_Numeric_Std_Gt_Sgn_Sgn,
    Ieee_Numeric_Std_Gt_Sgn_Int,
    Ieee_Numeric_Std_Gt_Int_Sgn,
    Ieee_Numeric_Std_Lt_Uns_Uns,
    Ieee_Numeric_Std_Lt_Uns_Nat,
    Ieee_Numeric_Std_Lt_Nat_Uns,
    Ieee_Numeric_Std_Lt_Sgn_Sgn,
    Ieee_Numeric_Std_Lt_Sgn_Int,
    Ieee_Numeric_Std_Lt_Int_Sgn,
    Ieee_Numeric_Std_Le_Uns_Uns,
    Ieee_Numeric_Std_Le_Uns_Nat,
    Ieee_Numeric_Std_Le_Nat_Uns,
    Ieee_Numeric_Std_Le_Sgn_Sgn,
    Ieee_Numeric_Std_Le_Sgn_Int,
    Ieee_Numeric_Std_Le_Int_Sgn,
    Ieee_Numeric_Std_Ge_Uns_Uns,
    Ieee_Numeric_Std_Ge_Uns_Nat,
    Ieee_Numeric_Std_Ge_Nat_Uns,
    Ieee_Numeric_Std_Ge_Sgn_Sgn,
    Ieee_Numeric_Std_Ge_Sgn_Int,
    Ieee_Numeric_Std_Ge_Int_Sgn,
    Ieee_Numeric_Std_Eq_Uns_Uns,
    Ieee_Numeric_Std_Eq_Uns_Nat,
    Ieee_Numeric_Std_Eq_Nat_Uns,
    Ieee_Numeric_Std_Eq_Sgn_Sgn,
    Ieee_Numeric_Std_Eq_Sgn_Int,
    Ieee_Numeric_Std_Eq_Int_Sgn,
    Ieee_Numeric_Std_Ne_Uns_Uns,
    Ieee_Numeric_Std_Ne_Uns_Nat,
    Ieee_Numeric_Std_Ne_Nat_Uns,
    Ieee_Numeric_Std_Ne_Sgn_Sgn,
    Ieee_Numeric_Std_Ne_Sgn_Int,
    Ieee_Numeric_Std_Ne_Int_Sgn,
    Ieee_Numeric_Std_Match_Gt_Uns_Uns,
    Ieee_Numeric_Std_Match_Gt_Uns_Nat,
    Ieee_Numeric_Std_Match_Gt_Nat_Uns,
    Ieee_Numeric_Std_Match_Gt_Sgn_Sgn,
    Ieee_Numeric_Std_Match_Gt_Sgn_Int,
    Ieee_Numeric_Std_Match_Gt_Int_Sgn,
    Ieee_Numeric_Std_Match_Lt_Uns_Uns,
    Ieee_Numeric_Std_Match_Lt_Uns_Nat,
    Ieee_Numeric_Std_Match_Lt_Nat_Uns,
    Ieee_Numeric_Std_Match_Lt_Sgn_Sgn,
    Ieee_Numeric_Std_Match_Lt_Sgn_Int,
    Ieee_Numeric_Std_Match_Lt_Int_Sgn,
    Ieee_Numeric_Std_Match_Le_Uns_Uns,
    Ieee_Numeric_Std_Match_Le_Uns_Nat,
    Ieee_Numeric_Std_Match_Le_Nat_Uns,
    Ieee_Numeric_Std_Match_Le_Sgn_Sgn,
    Ieee_Numeric_Std_Match_Le_Sgn_Int,
    Ieee_Numeric_Std_Match_Le_Int_Sgn,
    Ieee_Numeric_Std_Match_Ge_Uns_Uns,
    Ieee_Numeric_Std_Match_Ge_Uns_Nat,
    Ieee_Numeric_Std_Match_Ge_Nat_Uns,
    Ieee_Numeric_Std_Match_Ge_Sgn_Sgn,
    Ieee_Numeric_Std_Match_Ge_Sgn_Int,
    Ieee_Numeric_Std_Match_Ge_Int_Sgn,
    Ieee_Numeric_Std_Match_Eq_Uns_Uns,
    Ieee_Numeric_Std_Match_Eq_Uns_Nat,
    Ieee_Numeric_Std_Match_Eq_Nat_Uns,
    Ieee_Numeric_Std_Match_Eq_Sgn_Sgn,
    Ieee_Numeric_Std_Match_Eq_Sgn_Int,
    Ieee_Numeric_Std_Match_Eq_Int_Sgn,
    Ieee_Numeric_Std_Match_Ne_Uns_Uns,
    Ieee_Numeric_Std_Match_Ne_Uns_Nat,
    Ieee_Numeric_Std_Match_Ne_Nat_Uns,
    Ieee_Numeric_Std_Match_Ne_Sgn_Sgn,
    Ieee_Numeric_Std_Match_Ne_Sgn_Int,
    Ieee_Numeric_Std_Match_Ne_Int_Sgn,
    Ieee_Numeric_Std_Sll_Uns_Int,
    Ieee_Numeric_Std_Sll_Sgn_Int,
    Ieee_Numeric_Std_Srl_Uns_Int,
    Ieee_Numeric_Std_Srl_Sgn_Int,
    Ieee_Numeric_Std_Sla_Uns_Int,
    Ieee_Numeric_Std_Sla_Sgn_Int,
    Ieee_Numeric_Std_Sra_Uns_Int,
    Ieee_Numeric_Std_Sra_Sgn_Int,
    Ieee_Numeric_Std_Rol_Uns_Int,
    Ieee_Numeric_Std_Rol_Sgn_Int,
    Ieee_Numeric_Std_Ror_Uns_Int,
    Ieee_Numeric_Std_Ror_Sgn_Int,
    Ieee_Numeric_Std_And_Uns_Uns,
    Ieee_Numeric_Std_And_Uns_Log,
    Ieee_Numeric_Std_And_Log_Uns,
    Ieee_Numeric_Std_And_Sgn_Sgn,
    Ieee_Numeric_Std_And_Sgn_Log,
    Ieee_Numeric_Std_And_Log_Sgn,
    Ieee_Numeric_Std_Nand_Uns_Uns,
    Ieee_Numeric_Std_Nand_Uns_Log,
    Ieee_Numeric_Std_Nand_Log_Uns,
    Ieee_Numeric_Std_Nand_Sgn_Sgn,
    Ieee_Numeric_Std_Nand_Sgn_Log,
    Ieee_Numeric_Std_Nand_Log_Sgn,
    Ieee_Numeric_Std_Or_Uns_Uns,
    Ieee_Numeric_Std_Or_Uns_Log,
    Ieee_Numeric_Std_Or_Log_Uns,
    Ieee_Numeric_Std_Or_Sgn_Sgn,
    Ieee_Numeric_Std_Or_Sgn_Log,
    Ieee_Numeric_Std_Or_Log_Sgn,
    Ieee_Numeric_Std_Nor_Uns_Uns,
    Ieee_Numeric_Std_Nor_Uns_Log,
    Ieee_Numeric_Std_Nor_Log_Uns,
    Ieee_Numeric_Std_Nor_Sgn_Sgn,
    Ieee_Numeric_Std_Nor_Sgn_Log,
    Ieee_Numeric_Std_Nor_Log_Sgn,
    Ieee_Numeric_Std_Xor_Uns_Uns,
    Ieee_Numeric_Std_Xor_Uns_Log,
    Ieee_Numeric_Std_Xor_Log_Uns,
    Ieee_Numeric_Std_Xor_Sgn_Sgn,
    Ieee_Numeric_Std_Xor_Sgn_Log,
    Ieee_Numeric_Std_Xor_Log_Sgn,
    Ieee_Numeric_Std_Xnor_Uns_Uns,
    Ieee_Numeric_Std_Xnor_Uns_Log,
    Ieee_Numeric_Std_Xnor_Log_Uns,
    Ieee_Numeric_Std_Xnor_Sgn_Sgn,
    Ieee_Numeric_Std_Xnor_Sgn_Log,
    Ieee_Numeric_Std_Xnor_Log_Sgn,
    Ieee_Numeric_Std_Not_Uns,
    Ieee_Numeric_Std_Not_Sgn,
    Ieee_Numeric_Std_Abs_Sgn,
    Ieee_Numeric_Std_Neg_Uns,
    Ieee_Numeric_Std_Neg_Sgn,
    Ieee_Numeric_Std_Min_Uns_Uns,
    Ieee_Numeric_Std_Min_Uns_Nat,
    Ieee_Numeric_Std_Min_Nat_Uns,
    Ieee_Numeric_Std_Min_Sgn_Sgn,
    Ieee_Numeric_Std_Min_Sgn_Int,
    Ieee_Numeric_Std_Min_Int_Sgn,
    Ieee_Numeric_Std_Max_Uns_Uns,
    Ieee_Numeric_Std_Max_Uns_Nat,
    Ieee_Numeric_Std_Max_Nat_Uns,
    Ieee_Numeric_Std_Max_Sgn_Sgn,
    Ieee_Numeric_Std_Max_Sgn_Int,
    Ieee_Numeric_Std_Max_Int_Sgn,
    Ieee_Numeric_Std_Shf_Left_Uns_Nat,
    Ieee_Numeric_Std_Shf_Right_Uns_Nat,
    Ieee_Numeric_Std_Shf_Left_Sgn_Nat,
    Ieee_Numeric_Std_Shf_Right_Sgn_Nat,
    Ieee_Numeric_Std_Rot_Left_Uns_Nat,
    Ieee_Numeric_Std_Rot_Right_Uns_Nat,
    Ieee_Numeric_Std_Rot_Left_Sgn_Nat,
    Ieee_Numeric_Std_Rot_Right_Sgn_Nat,
    Ieee_Numeric_Std_And_Sgn,
    Ieee_Numeric_Std_Nand_Sgn,
    Ieee_Numeric_Std_Or_Sgn,
    Ieee_Numeric_Std_Nor_Sgn,
    Ieee_Numeric_Std_Xor_Sgn,
    Ieee_Numeric_Std_Xnor_Sgn,
    Ieee_Numeric_Std_And_Uns,
    Ieee_Numeric_Std_Nand_Uns,
    Ieee_Numeric_Std_Or_Uns,
    Ieee_Numeric_Std_Nor_Uns,
    Ieee_Numeric_Std_Xor_Uns,
    Ieee_Numeric_Std_Xnor_Uns,
    Ieee_Numeric_Std_Find_Leftmost_Uns,
    Ieee_Numeric_Std_Find_Rightmost_Uns,
    Ieee_Numeric_Std_Find_Leftmost_Sgn,
    Ieee_Numeric_Std_Find_Rightmost_Sgn,
    Ieee_Numeric_Std_Match_Log,
    Ieee_Numeric_Std_Match_Uns,
    Ieee_Numeric_Std_Match_Sgn,
    Ieee_Numeric_Std_Match_Slv,
    Ieee_Numeric_Std_Match_Suv,
    Ieee_Numeric_Std_To_01_Uns,
    Ieee_Numeric_Std_To_01_Sgn,
    Ieee_Numeric_Std_To_X01_Uns,
    Ieee_Numeric_Std_To_X01_Sgn,
    Ieee_Numeric_Std_To_X01Z_Uns,
    Ieee_Numeric_Std_To_X01Z_Sgn,
    Ieee_Numeric_Std_To_UX01_Uns,
    Ieee_Numeric_Std_To_UX01_Sgn,
    Ieee_Numeric_Std_Is_X_Uns,
    Ieee_Numeric_Std_Is_X_Sgn,
    Ieee_Numeric_Std_To_Hstring_Uns,
    Ieee_Numeric_Std_To_Ostring_Uns,
    Ieee_Numeric_Std_To_Hstring_Sgn,
    Ieee_Numeric_Std_To_Ostring_Sgn,
    Ieee_Numeric_Bit_Toint_Uns_Nat,
    Ieee_Numeric_Bit_Toint_Sgn_Int,
    Ieee_Numeric_Bit_Touns_Nat_Nat_Uns,
    Ieee_Numeric_Bit_Touns_Nat_Uns_Uns,
    Ieee_Numeric_Bit_Tosgn_Int_Nat_Sgn,
    Ieee_Numeric_Bit_Tosgn_Int_Sgn_Sgn,
    Ieee_Numeric_Std_Unsigned_Add_Slv_Slv,
    Ieee_Numeric_Std_Unsigned_Add_Slv_Nat,
    Ieee_Numeric_Std_Unsigned_Add_Nat_Slv,
    Ieee_Numeric_Std_Unsigned_Sub_Slv_Slv,
    Ieee_Numeric_Std_Unsigned_Sub_Slv_Nat,
    Ieee_Numeric_Std_Unsigned_Sub_Nat_Slv,
    Ieee_Numeric_Std_Unsigned_Find_Rightmost,
    Ieee_Numeric_Std_Unsigned_Find_Leftmost,
    Ieee_Numeric_Std_Unsigned_Shift_Left,
    Ieee_Numeric_Std_Unsigned_Shift_Right,
    Ieee_Numeric_Std_Unsigned_Rotate_Left,
    Ieee_Numeric_Std_Unsigned_Rotate_Right,
    Ieee_Numeric_Std_Unsigned_To_Integer_Slv_Nat,
    Ieee_Numeric_Std_Unsigned_To_Slv_Nat_Nat,
    Ieee_Numeric_Std_Unsigned_To_Slv_Nat_Slv,
    Ieee_Numeric_Std_Unsigned_To_Suv_Nat_Nat,
    Ieee_Numeric_Std_Unsigned_To_Suv_Nat_Suv,
    Ieee_Numeric_Std_Unsigned_Resize_Slv_Nat,
    Ieee_Numeric_Std_Unsigned_Resize_Slv_Slv,
    Ieee_Numeric_Std_Unsigned_Maximum_Slv_Slv,
    Ieee_Numeric_Std_Unsigned_Minimum_Slv_Slv,
    Ieee_Math_Real_Sign,
    Ieee_Math_Real_Ceil,
    Ieee_Math_Real_Floor,
    Ieee_Math_Real_Round,
    Ieee_Math_Real_Trunc,
    Ieee_Math_Real_Mod,
    Ieee_Math_Real_Realmax,
    Ieee_Math_Real_Realmin,
    Ieee_Math_Real_Sqrt,
    Ieee_Math_Real_Cbrt,
    Ieee_Math_Real_Pow_Int_Real,
    Ieee_Math_Real_Pow_Real_Real,
    Ieee_Math_Real_Exp,
    Ieee_Math_Real_Log,
    Ieee_Math_Real_Log2,
    Ieee_Math_Real_Log10,
    Ieee_Math_Real_Log_Real_Real,
    Ieee_Math_Real_Sin,
    Ieee_Math_Real_Cos,
    Ieee_Math_Real_Tan,
    Ieee_Math_Real_Arcsin,
    Ieee_Math_Real_Arccos,
    Ieee_Math_Real_Arctan,
    Ieee_Math_Real_Arctan_Real_Real,
    Ieee_Math_Real_Sinh,
    Ieee_Math_Real_Cosh,
    Ieee_Math_Real_Tanh,
    Ieee_Math_Real_Arcsinh,
    Ieee_Math_Real_Arccosh,
    Ieee_Math_Real_Arctanh,
    Ieee_Std_Logic_Unsigned_Add_Slv_Slv,
    Ieee_Std_Logic_Unsigned_Add_Slv_Int,
    Ieee_Std_Logic_Unsigned_Add_Int_Slv,
    Ieee_Std_Logic_Unsigned_Add_Slv_Log,
    Ieee_Std_Logic_Unsigned_Add_Log_Slv,
    Ieee_Std_Logic_Unsigned_Sub_Slv_Slv,
    Ieee_Std_Logic_Unsigned_Sub_Slv_Int,
    Ieee_Std_Logic_Unsigned_Sub_Int_Slv,
    Ieee_Std_Logic_Unsigned_Sub_Slv_Log,
    Ieee_Std_Logic_Unsigned_Sub_Log_Slv,
    Ieee_Std_Logic_Unsigned_Id_Slv,
    Ieee_Std_Logic_Unsigned_Mul_Slv_Slv,
    Ieee_Std_Logic_Unsigned_Lt_Slv_Slv,
    Ieee_Std_Logic_Unsigned_Lt_Slv_Int,
    Ieee_Std_Logic_Unsigned_Lt_Int_Slv,
    Ieee_Std_Logic_Unsigned_Le_Slv_Slv,
    Ieee_Std_Logic_Unsigned_Le_Slv_Int,
    Ieee_Std_Logic_Unsigned_Le_Int_Slv,
    Ieee_Std_Logic_Unsigned_Gt_Slv_Slv,
    Ieee_Std_Logic_Unsigned_Gt_Slv_Int,
    Ieee_Std_Logic_Unsigned_Gt_Int_Slv,
    Ieee_Std_Logic_Unsigned_Ge_Slv_Slv,
    Ieee_Std_Logic_Unsigned_Ge_Slv_Int,
    Ieee_Std_Logic_Unsigned_Ge_Int_Slv,
    Ieee_Std_Logic_Unsigned_Eq_Slv_Slv,
    Ieee_Std_Logic_Unsigned_Eq_Slv_Int,
    Ieee_Std_Logic_Unsigned_Eq_Int_Slv,
    Ieee_Std_Logic_Unsigned_Ne_Slv_Slv,
    Ieee_Std_Logic_Unsigned_Ne_Slv_Int,
    Ieee_Std_Logic_Unsigned_Ne_Int_Slv,
    Ieee_Std_Logic_Unsigned_Conv_Integer,
    Ieee_Std_Logic_Unsigned_Shl,
    Ieee_Std_Logic_Unsigned_Shr,
    Ieee_Std_Logic_Signed_Add_Slv_Slv,
    Ieee_Std_Logic_Signed_Add_Slv_Int,
    Ieee_Std_Logic_Signed_Add_Int_Slv,
    Ieee_Std_Logic_Signed_Add_Slv_Log,
    Ieee_Std_Logic_Signed_Add_Log_Slv,
    Ieee_Std_Logic_Signed_Sub_Slv_Slv,
    Ieee_Std_Logic_Signed_Sub_Slv_Int,
    Ieee_Std_Logic_Signed_Sub_Int_Slv,
    Ieee_Std_Logic_Signed_Sub_Slv_Log,
    Ieee_Std_Logic_Signed_Sub_Log_Slv,
    Ieee_Std_Logic_Signed_Id_Slv,
    Ieee_Std_Logic_Signed_Neg_Slv,
    Ieee_Std_Logic_Signed_Abs_Slv,
    Ieee_Std_Logic_Signed_Mul_Slv_Slv,
    Ieee_Std_Logic_Signed_Lt_Slv_Slv,
    Ieee_Std_Logic_Signed_Lt_Slv_Int,
    Ieee_Std_Logic_Signed_Lt_Int_Slv,
    Ieee_Std_Logic_Signed_Le_Slv_Slv,
    Ieee_Std_Logic_Signed_Le_Slv_Int,
    Ieee_Std_Logic_Signed_Le_Int_Slv,
    Ieee_Std_Logic_Signed_Gt_Slv_Slv,
    Ieee_Std_Logic_Signed_Gt_Slv_Int,
    Ieee_Std_Logic_Signed_Gt_Int_Slv,
    Ieee_Std_Logic_Signed_Ge_Slv_Slv,
    Ieee_Std_Logic_Signed_Ge_Slv_Int,
    Ieee_Std_Logic_Signed_Ge_Int_Slv,
    Ieee_Std_Logic_Signed_Eq_Slv_Slv,
    Ieee_Std_Logic_Signed_Eq_Slv_Int,
    Ieee_Std_Logic_Signed_Eq_Int_Slv,
    Ieee_Std_Logic_Signed_Ne_Slv_Slv,
    Ieee_Std_Logic_Signed_Ne_Slv_Int,
    Ieee_Std_Logic_Signed_Ne_Int_Slv,
    Ieee_Std_Logic_Signed_Conv_Integer,
    Ieee_Std_Logic_Signed_Shl,
    Ieee_Std_Logic_Signed_Shr,
    Ieee_Std_Logic_Arith_Conv_Unsigned_Int,
    Ieee_Std_Logic_Arith_Conv_Unsigned_Uns,
    Ieee_Std_Logic_Arith_Conv_Unsigned_Sgn,
    Ieee_Std_Logic_Arith_Conv_Unsigned_Log,
    Ieee_Std_Logic_Arith_Conv_Signed_Int,
    Ieee_Std_Logic_Arith_Conv_Signed_Uns,
    Ieee_Std_Logic_Arith_Conv_Signed_Sgn,
    Ieee_Std_Logic_Arith_Conv_Signed_Log,
    Ieee_Std_Logic_Arith_Conv_Integer_Int,
    Ieee_Std_Logic_Arith_Conv_Integer_Uns,
    Ieee_Std_Logic_Arith_Conv_Integer_Sgn,
    Ieee_Std_Logic_Arith_Conv_Integer_Log,
    Ieee_Std_Logic_Arith_Conv_Vector_Int,
    Ieee_Std_Logic_Arith_Conv_Vector_Uns,
    Ieee_Std_Logic_Arith_Conv_Vector_Sgn,
    Ieee_Std_Logic_Arith_Conv_Vector_Log,
    Ieee_Std_Logic_Arith_Ext,
    Ieee_Std_Logic_Arith_Sxt,
    Ieee_Std_Logic_Arith_Id_Uns_Uns,
    Ieee_Std_Logic_Arith_Id_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Neg_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Abs_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Shl_Uns,
    Ieee_Std_Logic_Arith_Shl_Sgn,
    Ieee_Std_Logic_Arith_Shr_Uns,
    Ieee_Std_Logic_Arith_Shr_Sgn,
    Ieee_Std_Logic_Arith_Id_Uns_Slv,
    Ieee_Std_Logic_Arith_Id_Sgn_Slv,
    Ieee_Std_Logic_Arith_Neg_Sgn_Slv,
    Ieee_Std_Logic_Arith_Abs_Sgn_Slv,
    Ieee_Std_Logic_Arith_Mul_Uns_Uns_Uns,
    Ieee_Std_Logic_Arith_Mul_Sgn_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Mul_Sgn_Uns_Sgn,
    Ieee_Std_Logic_Arith_Mul_Uns_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Mul_Uns_Uns_Slv,
    Ieee_Std_Logic_Arith_Mul_Sgn_Sgn_Slv,
    Ieee_Std_Logic_Arith_Mul_Sgn_Uns_Slv,
    Ieee_Std_Logic_Arith_Mul_Uns_Sgn_Slv,
    Ieee_Std_Logic_Arith_Add_Uns_Uns_Uns,
    Ieee_Std_Logic_Arith_Add_Sgn_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Add_Uns_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Add_Sgn_Uns_Sgn,
    Ieee_Std_Logic_Arith_Add_Uns_Int_Uns,
    Ieee_Std_Logic_Arith_Add_Int_Uns_Uns,
    Ieee_Std_Logic_Arith_Add_Sgn_Int_Sgn,
    Ieee_Std_Logic_Arith_Add_Int_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Add_Uns_Log_Uns,
    Ieee_Std_Logic_Arith_Add_Log_Uns_Uns,
    Ieee_Std_Logic_Arith_Add_Sgn_Log_Sgn,
    Ieee_Std_Logic_Arith_Add_Log_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Add_Uns_Uns_Slv,
    Ieee_Std_Logic_Arith_Add_Sgn_Sgn_Slv,
    Ieee_Std_Logic_Arith_Add_Uns_Sgn_Slv,
    Ieee_Std_Logic_Arith_Add_Sgn_Uns_Slv,
    Ieee_Std_Logic_Arith_Add_Uns_Int_Slv,
    Ieee_Std_Logic_Arith_Add_Int_Uns_Slv,
    Ieee_Std_Logic_Arith_Add_Sgn_Int_Slv,
    Ieee_Std_Logic_Arith_Add_Int_Sgn_Slv,
    Ieee_Std_Logic_Arith_Add_Uns_Log_Slv,
    Ieee_Std_Logic_Arith_Add_Log_Uns_Slv,
    Ieee_Std_Logic_Arith_Add_Sgn_Log_Slv,
    Ieee_Std_Logic_Arith_Add_Log_Sgn_Slv,
    Ieee_Std_Logic_Arith_Sub_Uns_Uns_Uns,
    Ieee_Std_Logic_Arith_Sub_Sgn_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Sub_Uns_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Sub_Sgn_Uns_Sgn,
    Ieee_Std_Logic_Arith_Sub_Uns_Int_Uns,
    Ieee_Std_Logic_Arith_Sub_Int_Uns_Uns,
    Ieee_Std_Logic_Arith_Sub_Sgn_Int_Sgn,
    Ieee_Std_Logic_Arith_Sub_Int_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Sub_Uns_Log_Uns,
    Ieee_Std_Logic_Arith_Sub_Log_Uns_Uns,
    Ieee_Std_Logic_Arith_Sub_Sgn_Log_Sgn,
    Ieee_Std_Logic_Arith_Sub_Log_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Sub_Uns_Uns_Slv,
    Ieee_Std_Logic_Arith_Sub_Sgn_Sgn_Slv,
    Ieee_Std_Logic_Arith_Sub_Uns_Sgn_Slv,
    Ieee_Std_Logic_Arith_Sub_Sgn_Uns_Slv,
    Ieee_Std_Logic_Arith_Sub_Uns_Int_Slv,
    Ieee_Std_Logic_Arith_Sub_Int_Uns_Slv,
    Ieee_Std_Logic_Arith_Sub_Sgn_Int_Slv,
    Ieee_Std_Logic_Arith_Sub_Int_Sgn_Slv,
    Ieee_Std_Logic_Arith_Sub_Uns_Log_Slv,
    Ieee_Std_Logic_Arith_Sub_Log_Uns_Slv,
    Ieee_Std_Logic_Arith_Sub_Sgn_Log_Slv,
    Ieee_Std_Logic_Arith_Sub_Log_Sgn_Slv,
    Ieee_Std_Logic_Arith_Lt_Uns_Uns,
    Ieee_Std_Logic_Arith_Lt_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Lt_Uns_Sgn,
    Ieee_Std_Logic_Arith_Lt_Sgn_Uns,
    Ieee_Std_Logic_Arith_Lt_Uns_Int,
    Ieee_Std_Logic_Arith_Lt_Int_Uns,
    Ieee_Std_Logic_Arith_Lt_Sgn_Int,
    Ieee_Std_Logic_Arith_Lt_Int_Sgn,
    Ieee_Std_Logic_Arith_Le_Uns_Uns,
    Ieee_Std_Logic_Arith_Le_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Le_Uns_Sgn,
    Ieee_Std_Logic_Arith_Le_Sgn_Uns,
    Ieee_Std_Logic_Arith_Le_Uns_Int,
    Ieee_Std_Logic_Arith_Le_Int_Uns,
    Ieee_Std_Logic_Arith_Le_Sgn_Int,
    Ieee_Std_Logic_Arith_Le_Int_Sgn,
    Ieee_Std_Logic_Arith_Gt_Uns_Uns,
    Ieee_Std_Logic_Arith_Gt_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Gt_Uns_Sgn,
    Ieee_Std_Logic_Arith_Gt_Sgn_Uns,
    Ieee_Std_Logic_Arith_Gt_Uns_Int,
    Ieee_Std_Logic_Arith_Gt_Int_Uns,
    Ieee_Std_Logic_Arith_Gt_Sgn_Int,
    Ieee_Std_Logic_Arith_Gt_Int_Sgn,
    Ieee_Std_Logic_Arith_Ge_Uns_Uns,
    Ieee_Std_Logic_Arith_Ge_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Ge_Uns_Sgn,
    Ieee_Std_Logic_Arith_Ge_Sgn_Uns,
    Ieee_Std_Logic_Arith_Ge_Uns_Int,
    Ieee_Std_Logic_Arith_Ge_Int_Uns,
    Ieee_Std_Logic_Arith_Ge_Sgn_Int,
    Ieee_Std_Logic_Arith_Ge_Int_Sgn,
    Ieee_Std_Logic_Arith_Eq_Uns_Uns,
    Ieee_Std_Logic_Arith_Eq_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Eq_Uns_Sgn,
    Ieee_Std_Logic_Arith_Eq_Sgn_Uns,
    Ieee_Std_Logic_Arith_Eq_Uns_Int,
    Ieee_Std_Logic_Arith_Eq_Int_Uns,
    Ieee_Std_Logic_Arith_Eq_Sgn_Int,
    Ieee_Std_Logic_Arith_Eq_Int_Sgn,
    Ieee_Std_Logic_Arith_Ne_Uns_Uns,
    Ieee_Std_Logic_Arith_Ne_Sgn_Sgn,
    Ieee_Std_Logic_Arith_Ne_Uns_Sgn,
    Ieee_Std_Logic_Arith_Ne_Sgn_Uns,
    Ieee_Std_Logic_Arith_Ne_Uns_Int,
    Ieee_Std_Logic_Arith_Ne_Int_Uns,
    Ieee_Std_Logic_Arith_Ne_Sgn_Int,
    Ieee_Std_Logic_Arith_Ne_Int_Sgn,
    Ieee_Std_Logic_Misc_And_Reduce_Slv,
    Ieee_Std_Logic_Misc_And_Reduce_Suv,
    Ieee_Std_Logic_Misc_Nand_Reduce_Slv,
    Ieee_Std_Logic_Misc_Nand_Reduce_Suv,
    Ieee_Std_Logic_Misc_Or_Reduce_Slv,
    Ieee_Std_Logic_Misc_Or_Reduce_Suv,
    Ieee_Std_Logic_Misc_Nor_Reduce_Slv,
    Ieee_Std_Logic_Misc_Nor_Reduce_Suv,
    Ieee_Std_Logic_Misc_Xor_Reduce_Slv,
    Ieee_Std_Logic_Misc_Xor_Reduce_Suv,
    Ieee_Std_Logic_Misc_Xnor_Reduce_Slv,
    Ieee_Std_Logic_Misc_Xnor_Reduce_Suv,
}

impl Iir_Predefined {
    const VALUES: [Self; 759] = [
        Self::Error,
        Self::Boolean_And,
        Self::Boolean_Or,
        Self::Boolean_Nand,
        Self::Boolean_Nor,
        Self::Boolean_Xor,
        Self::Boolean_Xnor,
        Self::Boolean_Not,
        Self::Boolean_Rising_Edge,
        Self::Boolean_Falling_Edge,
        Self::Enum_Equality,
        Self::Enum_Inequality,
        Self::Enum_Less,
        Self::Enum_Less_Equal,
        Self::Enum_Greater,
        Self::Enum_Greater_Equal,
        Self::Bit_And,
        Self::Bit_Or,
        Self::Bit_Nand,
        Self::Bit_Nor,
        Self::Bit_Xor,
        Self::Bit_Xnor,
        Self::Bit_Not,
        Self::Bit_Match_Equality,
        Self::Bit_Match_Inequality,
        Self::Bit_Match_Less,
        Self::Bit_Match_Less_Equal,
        Self::Bit_Match_Greater,
        Self::Bit_Match_Greater_Equal,
        Self::Bit_Condition,
        Self::Integer_Equality,
        Self::Integer_Inequality,
        Self::Integer_Less,
        Self::Integer_Less_Equal,
        Self::Integer_Greater,
        Self::Integer_Greater_Equal,
        Self::Integer_Identity,
        Self::Integer_Negation,
        Self::Integer_Absolute,
        Self::Integer_Plus,
        Self::Integer_Minus,
        Self::Integer_Mul,
        Self::Integer_Div,
        Self::Integer_Mod,
        Self::Integer_Rem,
        Self::Integer_Exp,
        Self::Floating_Equality,
        Self::Floating_Inequality,
        Self::Floating_Less,
        Self::Floating_Less_Equal,
        Self::Floating_Greater,
        Self::Floating_Greater_Equal,
        Self::Floating_Identity,
        Self::Floating_Negation,
        Self::Floating_Absolute,
        Self::Floating_Plus,
        Self::Floating_Minus,
        Self::Floating_Mul,
        Self::Floating_Div,
        Self::Floating_Exp,
        Self::Universal_R_I_Mul,
        Self::Universal_I_R_Mul,
        Self::Universal_R_I_Div,
        Self::Physical_Equality,
        Self::Physical_Inequality,
        Self::Physical_Less,
        Self::Physical_Less_Equal,
        Self::Physical_Greater,
        Self::Physical_Greater_Equal,
        Self::Physical_Identity,
        Self::Physical_Negation,
        Self::Physical_Absolute,
        Self::Physical_Plus,
        Self::Physical_Minus,
        Self::Physical_Integer_Mul,
        Self::Physical_Real_Mul,
        Self::Integer_Physical_Mul,
        Self::Real_Physical_Mul,
        Self::Physical_Integer_Div,
        Self::Physical_Real_Div,
        Self::Physical_Physical_Div,
        Self::Physical_Mod,
        Self::Physical_Rem,
        Self::Access_Equality,
        Self::Access_Inequality,
        Self::Record_Equality,
        Self::Record_Inequality,
        Self::Array_Equality,
        Self::Array_Inequality,
        Self::Array_Less,
        Self::Array_Less_Equal,
        Self::Array_Greater,
        Self::Array_Greater_Equal,
        Self::Array_Array_Concat,
        Self::Array_Element_Concat,
        Self::Element_Array_Concat,
        Self::Element_Element_Concat,
        Self::Array_Minimum,
        Self::Array_Maximum,
        Self::Vector_Minimum,
        Self::Vector_Maximum,
        Self::Array_Sll,
        Self::Array_Srl,
        Self::Array_Sla,
        Self::Array_Sra,
        Self::Array_Rol,
        Self::Array_Ror,
        Self::TF_Array_And,
        Self::TF_Array_Or,
        Self::TF_Array_Nand,
        Self::TF_Array_Nor,
        Self::TF_Array_Xor,
        Self::TF_Array_Xnor,
        Self::TF_Array_Not,
        Self::TF_Reduction_And,
        Self::TF_Reduction_Or,
        Self::TF_Reduction_Nand,
        Self::TF_Reduction_Nor,
        Self::TF_Reduction_Xor,
        Self::TF_Reduction_Xnor,
        Self::TF_Reduction_Not,
        Self::TF_Array_Element_And,
        Self::TF_Element_Array_And,
        Self::TF_Array_Element_Or,
        Self::TF_Element_Array_Or,
        Self::TF_Array_Element_Nand,
        Self::TF_Element_Array_Nand,
        Self::TF_Array_Element_Nor,
        Self::TF_Element_Array_Nor,
        Self::TF_Array_Element_Xor,
        Self::TF_Element_Array_Xor,
        Self::TF_Array_Element_Xnor,
        Self::TF_Element_Array_Xnor,
        Self::Bit_Array_Match_Equality,
        Self::Bit_Array_Match_Inequality,
        Self::Std_Ulogic_Match_Equality,
        Self::Std_Ulogic_Match_Inequality,
        Self::Std_Ulogic_Match_Less,
        Self::Std_Ulogic_Match_Less_Equal,
        Self::Std_Ulogic_Match_Greater,
        Self::Std_Ulogic_Match_Greater_Equal,
        Self::Std_Ulogic_Array_Match_Equality,
        Self::Std_Ulogic_Array_Match_Inequality,
        Self::Enum_Minimum,
        Self::Enum_Maximum,
        Self::Enum_To_String,
        Self::Integer_Minimum,
        Self::Integer_Maximum,
        Self::Integer_To_String,
        Self::Bit_Rising_Edge,
        Self::Bit_Falling_Edge,
        Self::Floating_Minimum,
        Self::Floating_Maximum,
        Self::Floating_To_String,
        Self::Real_To_String_Digits,
        Self::Real_To_String_Format,
        Self::Physical_Minimum,
        Self::Physical_Maximum,
        Self::Physical_To_String,
        Self::Time_To_String_Unit,
        Self::Array_Char_To_String,
        Self::Bit_Vector_To_Ostring,
        Self::Bit_Vector_To_Hstring,
        Self::Deallocate,
        Self::File_Open,
        Self::File_Open_Status,
        Self::File_Close,
        Self::Read,
        Self::Read_Length,
        Self::Flush,
        Self::Write,
        Self::Endfile,
        Self::Now_Function,
        Self::Real_Now_Function,
        Self::Frequency_Function,
        Self::None,
        Self::Foreign_Untruncated_Text_Read,
        Self::Foreign_Textio_Read_Real,
        Self::Foreign_Textio_Write_Real,
        Self::Std_Env_Stop_Status,
        Self::Std_Env_Stop,
        Self::Std_Env_Finish_Status,
        Self::Std_Env_Finish,
        Self::Std_Env_Resolution_Limit,
        Self::Ieee_1164_Scalar_And,
        Self::Ieee_1164_Scalar_Nand,
        Self::Ieee_1164_Scalar_Or,
        Self::Ieee_1164_Scalar_Nor,
        Self::Ieee_1164_Scalar_Xor,
        Self::Ieee_1164_Scalar_Xnor,
        Self::Ieee_1164_Scalar_Not,
        Self::Ieee_1164_Vector_And,
        Self::Ieee_1164_Vector_Nand,
        Self::Ieee_1164_Vector_Or,
        Self::Ieee_1164_Vector_Nor,
        Self::Ieee_1164_Vector_Xor,
        Self::Ieee_1164_Vector_Xnor,
        Self::Ieee_1164_Vector_Not,
        Self::Ieee_1164_To_Bit,
        Self::Ieee_1164_To_Bitvector,
        Self::Ieee_1164_To_Stdulogic,
        Self::Ieee_1164_To_Stdlogicvector_Bv,
        Self::Ieee_1164_To_Stdlogicvector_Suv,
        Self::Ieee_1164_To_Stdulogicvector_Bv,
        Self::Ieee_1164_To_Stdulogicvector_Slv,
        Self::Ieee_1164_To_X01_Slv,
        Self::Ieee_1164_To_X01_Suv,
        Self::Ieee_1164_To_X01_Log,
        Self::Ieee_1164_To_X01_Bv_Slv,
        Self::Ieee_1164_To_X01_Bv_Suv,
        Self::Ieee_1164_To_X01_Bit_Log,
        Self::Ieee_1164_To_X01Z_Slv,
        Self::Ieee_1164_To_X01Z_Suv,
        Self::Ieee_1164_To_X01Z_Log,
        Self::Ieee_1164_To_X01Z_Bv_Slv,
        Self::Ieee_1164_To_X01Z_Bv_Suv,
        Self::Ieee_1164_To_X01Z_Bit_Log,
        Self::Ieee_1164_To_UX01_Slv,
        Self::Ieee_1164_To_UX01_Suv,
        Self::Ieee_1164_To_UX01_Log,
        Self::Ieee_1164_To_UX01_Bv_Slv,
        Self::Ieee_1164_To_UX01_Bv_Suv,
        Self::Ieee_1164_To_UX01_Bit_Log,
        Self::Ieee_1164_Is_X_Slv,
        Self::Ieee_1164_Is_X_Log,
        Self::Ieee_1164_Rising_Edge,
        Self::Ieee_1164_Falling_Edge,
        Self::Ieee_1164_And_Suv_Log,
        Self::Ieee_1164_And_Log_Suv,
        Self::Ieee_1164_Nand_Suv_Log,
        Self::Ieee_1164_Nand_Log_Suv,
        Self::Ieee_1164_Or_Suv_Log,
        Self::Ieee_1164_Or_Log_Suv,
        Self::Ieee_1164_Nor_Suv_Log,
        Self::Ieee_1164_Nor_Log_Suv,
        Self::Ieee_1164_Xor_Suv_Log,
        Self::Ieee_1164_Xor_Log_Suv,
        Self::Ieee_1164_Xnor_Suv_Log,
        Self::Ieee_1164_Xnor_Log_Suv,
        Self::Ieee_1164_And_Suv,
        Self::Ieee_1164_Nand_Suv,
        Self::Ieee_1164_Or_Suv,
        Self::Ieee_1164_Nor_Suv,
        Self::Ieee_1164_Xor_Suv,
        Self::Ieee_1164_Xnor_Suv,
        Self::Ieee_1164_Vector_Sll,
        Self::Ieee_1164_Vector_Srl,
        Self::Ieee_1164_Vector_Rol,
        Self::Ieee_1164_Vector_Ror,
        Self::Ieee_1164_Condition_Operator,
        Self::Ieee_1164_To_01_Log_Log,
        Self::Ieee_1164_To_01_Slv_Log,
        Self::Ieee_1164_To_Hstring,
        Self::Ieee_1164_To_Ostring,
        Self::Ieee_Numeric_Std_Toint_Uns_Nat,
        Self::Ieee_Numeric_Std_Toint_Sgn_Int,
        Self::Ieee_Numeric_Std_Touns_Nat_Nat_Uns,
        Self::Ieee_Numeric_Std_Touns_Nat_Uns_Uns,
        Self::Ieee_Numeric_Std_Tosgn_Int_Nat_Sgn,
        Self::Ieee_Numeric_Std_Tosgn_Int_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Resize_Uns_Nat,
        Self::Ieee_Numeric_Std_Resize_Sgn_Nat,
        Self::Ieee_Numeric_Std_Resize_Uns_Uns,
        Self::Ieee_Numeric_Std_Resize_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Add_Uns_Uns,
        Self::Ieee_Numeric_Std_Add_Uns_Nat,
        Self::Ieee_Numeric_Std_Add_Nat_Uns,
        Self::Ieee_Numeric_Std_Add_Uns_Log,
        Self::Ieee_Numeric_Std_Add_Log_Uns,
        Self::Ieee_Numeric_Std_Add_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Add_Sgn_Int,
        Self::Ieee_Numeric_Std_Add_Int_Sgn,
        Self::Ieee_Numeric_Std_Add_Sgn_Log,
        Self::Ieee_Numeric_Std_Add_Log_Sgn,
        Self::Ieee_Numeric_Std_Sub_Uns_Uns,
        Self::Ieee_Numeric_Std_Sub_Uns_Nat,
        Self::Ieee_Numeric_Std_Sub_Nat_Uns,
        Self::Ieee_Numeric_Std_Sub_Uns_Log,
        Self::Ieee_Numeric_Std_Sub_Log_Uns,
        Self::Ieee_Numeric_Std_Sub_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Sub_Sgn_Int,
        Self::Ieee_Numeric_Std_Sub_Int_Sgn,
        Self::Ieee_Numeric_Std_Sub_Sgn_Log,
        Self::Ieee_Numeric_Std_Sub_Log_Sgn,
        Self::Ieee_Numeric_Std_Mul_Uns_Uns,
        Self::Ieee_Numeric_Std_Mul_Uns_Nat,
        Self::Ieee_Numeric_Std_Mul_Nat_Uns,
        Self::Ieee_Numeric_Std_Mul_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Mul_Sgn_Int,
        Self::Ieee_Numeric_Std_Mul_Int_Sgn,
        Self::Ieee_Numeric_Std_Div_Uns_Uns,
        Self::Ieee_Numeric_Std_Div_Uns_Nat,
        Self::Ieee_Numeric_Std_Div_Nat_Uns,
        Self::Ieee_Numeric_Std_Div_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Div_Sgn_Int,
        Self::Ieee_Numeric_Std_Div_Int_Sgn,
        Self::Ieee_Numeric_Std_Rem_Uns_Uns,
        Self::Ieee_Numeric_Std_Rem_Uns_Nat,
        Self::Ieee_Numeric_Std_Rem_Nat_Uns,
        Self::Ieee_Numeric_Std_Rem_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Rem_Sgn_Int,
        Self::Ieee_Numeric_Std_Rem_Int_Sgn,
        Self::Ieee_Numeric_Std_Mod_Uns_Uns,
        Self::Ieee_Numeric_Std_Mod_Uns_Nat,
        Self::Ieee_Numeric_Std_Mod_Nat_Uns,
        Self::Ieee_Numeric_Std_Mod_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Mod_Sgn_Int,
        Self::Ieee_Numeric_Std_Mod_Int_Sgn,
        Self::Ieee_Numeric_Std_Gt_Uns_Uns,
        Self::Ieee_Numeric_Std_Gt_Uns_Nat,
        Self::Ieee_Numeric_Std_Gt_Nat_Uns,
        Self::Ieee_Numeric_Std_Gt_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Gt_Sgn_Int,
        Self::Ieee_Numeric_Std_Gt_Int_Sgn,
        Self::Ieee_Numeric_Std_Lt_Uns_Uns,
        Self::Ieee_Numeric_Std_Lt_Uns_Nat,
        Self::Ieee_Numeric_Std_Lt_Nat_Uns,
        Self::Ieee_Numeric_Std_Lt_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Lt_Sgn_Int,
        Self::Ieee_Numeric_Std_Lt_Int_Sgn,
        Self::Ieee_Numeric_Std_Le_Uns_Uns,
        Self::Ieee_Numeric_Std_Le_Uns_Nat,
        Self::Ieee_Numeric_Std_Le_Nat_Uns,
        Self::Ieee_Numeric_Std_Le_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Le_Sgn_Int,
        Self::Ieee_Numeric_Std_Le_Int_Sgn,
        Self::Ieee_Numeric_Std_Ge_Uns_Uns,
        Self::Ieee_Numeric_Std_Ge_Uns_Nat,
        Self::Ieee_Numeric_Std_Ge_Nat_Uns,
        Self::Ieee_Numeric_Std_Ge_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Ge_Sgn_Int,
        Self::Ieee_Numeric_Std_Ge_Int_Sgn,
        Self::Ieee_Numeric_Std_Eq_Uns_Uns,
        Self::Ieee_Numeric_Std_Eq_Uns_Nat,
        Self::Ieee_Numeric_Std_Eq_Nat_Uns,
        Self::Ieee_Numeric_Std_Eq_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Eq_Sgn_Int,
        Self::Ieee_Numeric_Std_Eq_Int_Sgn,
        Self::Ieee_Numeric_Std_Ne_Uns_Uns,
        Self::Ieee_Numeric_Std_Ne_Uns_Nat,
        Self::Ieee_Numeric_Std_Ne_Nat_Uns,
        Self::Ieee_Numeric_Std_Ne_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Ne_Sgn_Int,
        Self::Ieee_Numeric_Std_Ne_Int_Sgn,
        Self::Ieee_Numeric_Std_Match_Gt_Uns_Uns,
        Self::Ieee_Numeric_Std_Match_Gt_Uns_Nat,
        Self::Ieee_Numeric_Std_Match_Gt_Nat_Uns,
        Self::Ieee_Numeric_Std_Match_Gt_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Match_Gt_Sgn_Int,
        Self::Ieee_Numeric_Std_Match_Gt_Int_Sgn,
        Self::Ieee_Numeric_Std_Match_Lt_Uns_Uns,
        Self::Ieee_Numeric_Std_Match_Lt_Uns_Nat,
        Self::Ieee_Numeric_Std_Match_Lt_Nat_Uns,
        Self::Ieee_Numeric_Std_Match_Lt_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Match_Lt_Sgn_Int,
        Self::Ieee_Numeric_Std_Match_Lt_Int_Sgn,
        Self::Ieee_Numeric_Std_Match_Le_Uns_Uns,
        Self::Ieee_Numeric_Std_Match_Le_Uns_Nat,
        Self::Ieee_Numeric_Std_Match_Le_Nat_Uns,
        Self::Ieee_Numeric_Std_Match_Le_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Match_Le_Sgn_Int,
        Self::Ieee_Numeric_Std_Match_Le_Int_Sgn,
        Self::Ieee_Numeric_Std_Match_Ge_Uns_Uns,
        Self::Ieee_Numeric_Std_Match_Ge_Uns_Nat,
        Self::Ieee_Numeric_Std_Match_Ge_Nat_Uns,
        Self::Ieee_Numeric_Std_Match_Ge_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Match_Ge_Sgn_Int,
        Self::Ieee_Numeric_Std_Match_Ge_Int_Sgn,
        Self::Ieee_Numeric_Std_Match_Eq_Uns_Uns,
        Self::Ieee_Numeric_Std_Match_Eq_Uns_Nat,
        Self::Ieee_Numeric_Std_Match_Eq_Nat_Uns,
        Self::Ieee_Numeric_Std_Match_Eq_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Match_Eq_Sgn_Int,
        Self::Ieee_Numeric_Std_Match_Eq_Int_Sgn,
        Self::Ieee_Numeric_Std_Match_Ne_Uns_Uns,
        Self::Ieee_Numeric_Std_Match_Ne_Uns_Nat,
        Self::Ieee_Numeric_Std_Match_Ne_Nat_Uns,
        Self::Ieee_Numeric_Std_Match_Ne_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Match_Ne_Sgn_Int,
        Self::Ieee_Numeric_Std_Match_Ne_Int_Sgn,
        Self::Ieee_Numeric_Std_Sll_Uns_Int,
        Self::Ieee_Numeric_Std_Sll_Sgn_Int,
        Self::Ieee_Numeric_Std_Srl_Uns_Int,
        Self::Ieee_Numeric_Std_Srl_Sgn_Int,
        Self::Ieee_Numeric_Std_Sla_Uns_Int,
        Self::Ieee_Numeric_Std_Sla_Sgn_Int,
        Self::Ieee_Numeric_Std_Sra_Uns_Int,
        Self::Ieee_Numeric_Std_Sra_Sgn_Int,
        Self::Ieee_Numeric_Std_Rol_Uns_Int,
        Self::Ieee_Numeric_Std_Rol_Sgn_Int,
        Self::Ieee_Numeric_Std_Ror_Uns_Int,
        Self::Ieee_Numeric_Std_Ror_Sgn_Int,
        Self::Ieee_Numeric_Std_And_Uns_Uns,
        Self::Ieee_Numeric_Std_And_Uns_Log,
        Self::Ieee_Numeric_Std_And_Log_Uns,
        Self::Ieee_Numeric_Std_And_Sgn_Sgn,
        Self::Ieee_Numeric_Std_And_Sgn_Log,
        Self::Ieee_Numeric_Std_And_Log_Sgn,
        Self::Ieee_Numeric_Std_Nand_Uns_Uns,
        Self::Ieee_Numeric_Std_Nand_Uns_Log,
        Self::Ieee_Numeric_Std_Nand_Log_Uns,
        Self::Ieee_Numeric_Std_Nand_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Nand_Sgn_Log,
        Self::Ieee_Numeric_Std_Nand_Log_Sgn,
        Self::Ieee_Numeric_Std_Or_Uns_Uns,
        Self::Ieee_Numeric_Std_Or_Uns_Log,
        Self::Ieee_Numeric_Std_Or_Log_Uns,
        Self::Ieee_Numeric_Std_Or_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Or_Sgn_Log,
        Self::Ieee_Numeric_Std_Or_Log_Sgn,
        Self::Ieee_Numeric_Std_Nor_Uns_Uns,
        Self::Ieee_Numeric_Std_Nor_Uns_Log,
        Self::Ieee_Numeric_Std_Nor_Log_Uns,
        Self::Ieee_Numeric_Std_Nor_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Nor_Sgn_Log,
        Self::Ieee_Numeric_Std_Nor_Log_Sgn,
        Self::Ieee_Numeric_Std_Xor_Uns_Uns,
        Self::Ieee_Numeric_Std_Xor_Uns_Log,
        Self::Ieee_Numeric_Std_Xor_Log_Uns,
        Self::Ieee_Numeric_Std_Xor_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Xor_Sgn_Log,
        Self::Ieee_Numeric_Std_Xor_Log_Sgn,
        Self::Ieee_Numeric_Std_Xnor_Uns_Uns,
        Self::Ieee_Numeric_Std_Xnor_Uns_Log,
        Self::Ieee_Numeric_Std_Xnor_Log_Uns,
        Self::Ieee_Numeric_Std_Xnor_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Xnor_Sgn_Log,
        Self::Ieee_Numeric_Std_Xnor_Log_Sgn,
        Self::Ieee_Numeric_Std_Not_Uns,
        Self::Ieee_Numeric_Std_Not_Sgn,
        Self::Ieee_Numeric_Std_Abs_Sgn,
        Self::Ieee_Numeric_Std_Neg_Uns,
        Self::Ieee_Numeric_Std_Neg_Sgn,
        Self::Ieee_Numeric_Std_Min_Uns_Uns,
        Self::Ieee_Numeric_Std_Min_Uns_Nat,
        Self::Ieee_Numeric_Std_Min_Nat_Uns,
        Self::Ieee_Numeric_Std_Min_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Min_Sgn_Int,
        Self::Ieee_Numeric_Std_Min_Int_Sgn,
        Self::Ieee_Numeric_Std_Max_Uns_Uns,
        Self::Ieee_Numeric_Std_Max_Uns_Nat,
        Self::Ieee_Numeric_Std_Max_Nat_Uns,
        Self::Ieee_Numeric_Std_Max_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Max_Sgn_Int,
        Self::Ieee_Numeric_Std_Max_Int_Sgn,
        Self::Ieee_Numeric_Std_Shf_Left_Uns_Nat,
        Self::Ieee_Numeric_Std_Shf_Right_Uns_Nat,
        Self::Ieee_Numeric_Std_Shf_Left_Sgn_Nat,
        Self::Ieee_Numeric_Std_Shf_Right_Sgn_Nat,
        Self::Ieee_Numeric_Std_Rot_Left_Uns_Nat,
        Self::Ieee_Numeric_Std_Rot_Right_Uns_Nat,
        Self::Ieee_Numeric_Std_Rot_Left_Sgn_Nat,
        Self::Ieee_Numeric_Std_Rot_Right_Sgn_Nat,
        Self::Ieee_Numeric_Std_And_Sgn,
        Self::Ieee_Numeric_Std_Nand_Sgn,
        Self::Ieee_Numeric_Std_Or_Sgn,
        Self::Ieee_Numeric_Std_Nor_Sgn,
        Self::Ieee_Numeric_Std_Xor_Sgn,
        Self::Ieee_Numeric_Std_Xnor_Sgn,
        Self::Ieee_Numeric_Std_And_Uns,
        Self::Ieee_Numeric_Std_Nand_Uns,
        Self::Ieee_Numeric_Std_Or_Uns,
        Self::Ieee_Numeric_Std_Nor_Uns,
        Self::Ieee_Numeric_Std_Xor_Uns,
        Self::Ieee_Numeric_Std_Xnor_Uns,
        Self::Ieee_Numeric_Std_Find_Leftmost_Uns,
        Self::Ieee_Numeric_Std_Find_Rightmost_Uns,
        Self::Ieee_Numeric_Std_Find_Leftmost_Sgn,
        Self::Ieee_Numeric_Std_Find_Rightmost_Sgn,
        Self::Ieee_Numeric_Std_Match_Log,
        Self::Ieee_Numeric_Std_Match_Uns,
        Self::Ieee_Numeric_Std_Match_Sgn,
        Self::Ieee_Numeric_Std_Match_Slv,
        Self::Ieee_Numeric_Std_Match_Suv,
        Self::Ieee_Numeric_Std_To_01_Uns,
        Self::Ieee_Numeric_Std_To_01_Sgn,
        Self::Ieee_Numeric_Std_To_X01_Uns,
        Self::Ieee_Numeric_Std_To_X01_Sgn,
        Self::Ieee_Numeric_Std_To_X01Z_Uns,
        Self::Ieee_Numeric_Std_To_X01Z_Sgn,
        Self::Ieee_Numeric_Std_To_UX01_Uns,
        Self::Ieee_Numeric_Std_To_UX01_Sgn,
        Self::Ieee_Numeric_Std_Is_X_Uns,
        Self::Ieee_Numeric_Std_Is_X_Sgn,
        Self::Ieee_Numeric_Std_To_Hstring_Uns,
        Self::Ieee_Numeric_Std_To_Ostring_Uns,
        Self::Ieee_Numeric_Std_To_Hstring_Sgn,
        Self::Ieee_Numeric_Std_To_Ostring_Sgn,
        Self::Ieee_Numeric_Bit_Toint_Uns_Nat,
        Self::Ieee_Numeric_Bit_Toint_Sgn_Int,
        Self::Ieee_Numeric_Bit_Touns_Nat_Nat_Uns,
        Self::Ieee_Numeric_Bit_Touns_Nat_Uns_Uns,
        Self::Ieee_Numeric_Bit_Tosgn_Int_Nat_Sgn,
        Self::Ieee_Numeric_Bit_Tosgn_Int_Sgn_Sgn,
        Self::Ieee_Numeric_Std_Unsigned_Add_Slv_Slv,
        Self::Ieee_Numeric_Std_Unsigned_Add_Slv_Nat,
        Self::Ieee_Numeric_Std_Unsigned_Add_Nat_Slv,
        Self::Ieee_Numeric_Std_Unsigned_Sub_Slv_Slv,
        Self::Ieee_Numeric_Std_Unsigned_Sub_Slv_Nat,
        Self::Ieee_Numeric_Std_Unsigned_Sub_Nat_Slv,
        Self::Ieee_Numeric_Std_Unsigned_Find_Rightmost,
        Self::Ieee_Numeric_Std_Unsigned_Find_Leftmost,
        Self::Ieee_Numeric_Std_Unsigned_Shift_Left,
        Self::Ieee_Numeric_Std_Unsigned_Shift_Right,
        Self::Ieee_Numeric_Std_Unsigned_Rotate_Left,
        Self::Ieee_Numeric_Std_Unsigned_Rotate_Right,
        Self::Ieee_Numeric_Std_Unsigned_To_Integer_Slv_Nat,
        Self::Ieee_Numeric_Std_Unsigned_To_Slv_Nat_Nat,
        Self::Ieee_Numeric_Std_Unsigned_To_Slv_Nat_Slv,
        Self::Ieee_Numeric_Std_Unsigned_To_Suv_Nat_Nat,
        Self::Ieee_Numeric_Std_Unsigned_To_Suv_Nat_Suv,
        Self::Ieee_Numeric_Std_Unsigned_Resize_Slv_Nat,
        Self::Ieee_Numeric_Std_Unsigned_Resize_Slv_Slv,
        Self::Ieee_Numeric_Std_Unsigned_Maximum_Slv_Slv,
        Self::Ieee_Numeric_Std_Unsigned_Minimum_Slv_Slv,
        Self::Ieee_Math_Real_Sign,
        Self::Ieee_Math_Real_Ceil,
        Self::Ieee_Math_Real_Floor,
        Self::Ieee_Math_Real_Round,
        Self::Ieee_Math_Real_Trunc,
        Self::Ieee_Math_Real_Mod,
        Self::Ieee_Math_Real_Realmax,
        Self::Ieee_Math_Real_Realmin,
        Self::Ieee_Math_Real_Sqrt,
        Self::Ieee_Math_Real_Cbrt,
        Self::Ieee_Math_Real_Pow_Int_Real,
        Self::Ieee_Math_Real_Pow_Real_Real,
        Self::Ieee_Math_Real_Exp,
        Self::Ieee_Math_Real_Log,
        Self::Ieee_Math_Real_Log2,
        Self::Ieee_Math_Real_Log10,
        Self::Ieee_Math_Real_Log_Real_Real,
        Self::Ieee_Math_Real_Sin,
        Self::Ieee_Math_Real_Cos,
        Self::Ieee_Math_Real_Tan,
        Self::Ieee_Math_Real_Arcsin,
        Self::Ieee_Math_Real_Arccos,
        Self::Ieee_Math_Real_Arctan,
        Self::Ieee_Math_Real_Arctan_Real_Real,
        Self::Ieee_Math_Real_Sinh,
        Self::Ieee_Math_Real_Cosh,
        Self::Ieee_Math_Real_Tanh,
        Self::Ieee_Math_Real_Arcsinh,
        Self::Ieee_Math_Real_Arccosh,
        Self::Ieee_Math_Real_Arctanh,
        Self::Ieee_Std_Logic_Unsigned_Add_Slv_Slv,
        Self::Ieee_Std_Logic_Unsigned_Add_Slv_Int,
        Self::Ieee_Std_Logic_Unsigned_Add_Int_Slv,
        Self::Ieee_Std_Logic_Unsigned_Add_Slv_Log,
        Self::Ieee_Std_Logic_Unsigned_Add_Log_Slv,
        Self::Ieee_Std_Logic_Unsigned_Sub_Slv_Slv,
        Self::Ieee_Std_Logic_Unsigned_Sub_Slv_Int,
        Self::Ieee_Std_Logic_Unsigned_Sub_Int_Slv,
        Self::Ieee_Std_Logic_Unsigned_Sub_Slv_Log,
        Self::Ieee_Std_Logic_Unsigned_Sub_Log_Slv,
        Self::Ieee_Std_Logic_Unsigned_Id_Slv,
        Self::Ieee_Std_Logic_Unsigned_Mul_Slv_Slv,
        Self::Ieee_Std_Logic_Unsigned_Lt_Slv_Slv,
        Self::Ieee_Std_Logic_Unsigned_Lt_Slv_Int,
        Self::Ieee_Std_Logic_Unsigned_Lt_Int_Slv,
        Self::Ieee_Std_Logic_Unsigned_Le_Slv_Slv,
        Self::Ieee_Std_Logic_Unsigned_Le_Slv_Int,
        Self::Ieee_Std_Logic_Unsigned_Le_Int_Slv,
        Self::Ieee_Std_Logic_Unsigned_Gt_Slv_Slv,
        Self::Ieee_Std_Logic_Unsigned_Gt_Slv_Int,
        Self::Ieee_Std_Logic_Unsigned_Gt_Int_Slv,
        Self::Ieee_Std_Logic_Unsigned_Ge_Slv_Slv,
        Self::Ieee_Std_Logic_Unsigned_Ge_Slv_Int,
        Self::Ieee_Std_Logic_Unsigned_Ge_Int_Slv,
        Self::Ieee_Std_Logic_Unsigned_Eq_Slv_Slv,
        Self::Ieee_Std_Logic_Unsigned_Eq_Slv_Int,
        Self::Ieee_Std_Logic_Unsigned_Eq_Int_Slv,
        Self::Ieee_Std_Logic_Unsigned_Ne_Slv_Slv,
        Self::Ieee_Std_Logic_Unsigned_Ne_Slv_Int,
        Self::Ieee_Std_Logic_Unsigned_Ne_Int_Slv,
        Self::Ieee_Std_Logic_Unsigned_Conv_Integer,
        Self::Ieee_Std_Logic_Unsigned_Shl,
        Self::Ieee_Std_Logic_Unsigned_Shr,
        Self::Ieee_Std_Logic_Signed_Add_Slv_Slv,
        Self::Ieee_Std_Logic_Signed_Add_Slv_Int,
        Self::Ieee_Std_Logic_Signed_Add_Int_Slv,
        Self::Ieee_Std_Logic_Signed_Add_Slv_Log,
        Self::Ieee_Std_Logic_Signed_Add_Log_Slv,
        Self::Ieee_Std_Logic_Signed_Sub_Slv_Slv,
        Self::Ieee_Std_Logic_Signed_Sub_Slv_Int,
        Self::Ieee_Std_Logic_Signed_Sub_Int_Slv,
        Self::Ieee_Std_Logic_Signed_Sub_Slv_Log,
        Self::Ieee_Std_Logic_Signed_Sub_Log_Slv,
        Self::Ieee_Std_Logic_Signed_Id_Slv,
        Self::Ieee_Std_Logic_Signed_Neg_Slv,
        Self::Ieee_Std_Logic_Signed_Abs_Slv,
        Self::Ieee_Std_Logic_Signed_Mul_Slv_Slv,
        Self::Ieee_Std_Logic_Signed_Lt_Slv_Slv,
        Self::Ieee_Std_Logic_Signed_Lt_Slv_Int,
        Self::Ieee_Std_Logic_Signed_Lt_Int_Slv,
        Self::Ieee_Std_Logic_Signed_Le_Slv_Slv,
        Self::Ieee_Std_Logic_Signed_Le_Slv_Int,
        Self::Ieee_Std_Logic_Signed_Le_Int_Slv,
        Self::Ieee_Std_Logic_Signed_Gt_Slv_Slv,
        Self::Ieee_Std_Logic_Signed_Gt_Slv_Int,
        Self::Ieee_Std_Logic_Signed_Gt_Int_Slv,
        Self::Ieee_Std_Logic_Signed_Ge_Slv_Slv,
        Self::Ieee_Std_Logic_Signed_Ge_Slv_Int,
        Self::Ieee_Std_Logic_Signed_Ge_Int_Slv,
        Self::Ieee_Std_Logic_Signed_Eq_Slv_Slv,
        Self::Ieee_Std_Logic_Signed_Eq_Slv_Int,
        Self::Ieee_Std_Logic_Signed_Eq_Int_Slv,
        Self::Ieee_Std_Logic_Signed_Ne_Slv_Slv,
        Self::Ieee_Std_Logic_Signed_Ne_Slv_Int,
        Self::Ieee_Std_Logic_Signed_Ne_Int_Slv,
        Self::Ieee_Std_Logic_Signed_Conv_Integer,
        Self::Ieee_Std_Logic_Signed_Shl,
        Self::Ieee_Std_Logic_Signed_Shr,
        Self::Ieee_Std_Logic_Arith_Conv_Unsigned_Int,
        Self::Ieee_Std_Logic_Arith_Conv_Unsigned_Uns,
        Self::Ieee_Std_Logic_Arith_Conv_Unsigned_Sgn,
        Self::Ieee_Std_Logic_Arith_Conv_Unsigned_Log,
        Self::Ieee_Std_Logic_Arith_Conv_Signed_Int,
        Self::Ieee_Std_Logic_Arith_Conv_Signed_Uns,
        Self::Ieee_Std_Logic_Arith_Conv_Signed_Sgn,
        Self::Ieee_Std_Logic_Arith_Conv_Signed_Log,
        Self::Ieee_Std_Logic_Arith_Conv_Integer_Int,
        Self::Ieee_Std_Logic_Arith_Conv_Integer_Uns,
        Self::Ieee_Std_Logic_Arith_Conv_Integer_Sgn,
        Self::Ieee_Std_Logic_Arith_Conv_Integer_Log,
        Self::Ieee_Std_Logic_Arith_Conv_Vector_Int,
        Self::Ieee_Std_Logic_Arith_Conv_Vector_Uns,
        Self::Ieee_Std_Logic_Arith_Conv_Vector_Sgn,
        Self::Ieee_Std_Logic_Arith_Conv_Vector_Log,
        Self::Ieee_Std_Logic_Arith_Ext,
        Self::Ieee_Std_Logic_Arith_Sxt,
        Self::Ieee_Std_Logic_Arith_Id_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Id_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Neg_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Abs_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Shl_Uns,
        Self::Ieee_Std_Logic_Arith_Shl_Sgn,
        Self::Ieee_Std_Logic_Arith_Shr_Uns,
        Self::Ieee_Std_Logic_Arith_Shr_Sgn,
        Self::Ieee_Std_Logic_Arith_Id_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Id_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Neg_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Abs_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Mul_Uns_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Mul_Sgn_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Mul_Sgn_Uns_Sgn,
        Self::Ieee_Std_Logic_Arith_Mul_Uns_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Mul_Uns_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Mul_Sgn_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Mul_Sgn_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Mul_Uns_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Uns_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Add_Sgn_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Add_Uns_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Add_Sgn_Uns_Sgn,
        Self::Ieee_Std_Logic_Arith_Add_Uns_Int_Uns,
        Self::Ieee_Std_Logic_Arith_Add_Int_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Add_Sgn_Int_Sgn,
        Self::Ieee_Std_Logic_Arith_Add_Int_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Add_Uns_Log_Uns,
        Self::Ieee_Std_Logic_Arith_Add_Log_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Add_Sgn_Log_Sgn,
        Self::Ieee_Std_Logic_Arith_Add_Log_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Add_Uns_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Sgn_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Uns_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Sgn_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Uns_Int_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Int_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Sgn_Int_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Int_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Uns_Log_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Log_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Sgn_Log_Slv,
        Self::Ieee_Std_Logic_Arith_Add_Log_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Uns_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Sub_Sgn_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Sub_Uns_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Sub_Sgn_Uns_Sgn,
        Self::Ieee_Std_Logic_Arith_Sub_Uns_Int_Uns,
        Self::Ieee_Std_Logic_Arith_Sub_Int_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Sub_Sgn_Int_Sgn,
        Self::Ieee_Std_Logic_Arith_Sub_Int_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Sub_Uns_Log_Uns,
        Self::Ieee_Std_Logic_Arith_Sub_Log_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Sub_Sgn_Log_Sgn,
        Self::Ieee_Std_Logic_Arith_Sub_Log_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Sub_Uns_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Sgn_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Uns_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Sgn_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Uns_Int_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Int_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Sgn_Int_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Int_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Uns_Log_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Log_Uns_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Sgn_Log_Slv,
        Self::Ieee_Std_Logic_Arith_Sub_Log_Sgn_Slv,
        Self::Ieee_Std_Logic_Arith_Lt_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Lt_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Lt_Uns_Sgn,
        Self::Ieee_Std_Logic_Arith_Lt_Sgn_Uns,
        Self::Ieee_Std_Logic_Arith_Lt_Uns_Int,
        Self::Ieee_Std_Logic_Arith_Lt_Int_Uns,
        Self::Ieee_Std_Logic_Arith_Lt_Sgn_Int,
        Self::Ieee_Std_Logic_Arith_Lt_Int_Sgn,
        Self::Ieee_Std_Logic_Arith_Le_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Le_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Le_Uns_Sgn,
        Self::Ieee_Std_Logic_Arith_Le_Sgn_Uns,
        Self::Ieee_Std_Logic_Arith_Le_Uns_Int,
        Self::Ieee_Std_Logic_Arith_Le_Int_Uns,
        Self::Ieee_Std_Logic_Arith_Le_Sgn_Int,
        Self::Ieee_Std_Logic_Arith_Le_Int_Sgn,
        Self::Ieee_Std_Logic_Arith_Gt_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Gt_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Gt_Uns_Sgn,
        Self::Ieee_Std_Logic_Arith_Gt_Sgn_Uns,
        Self::Ieee_Std_Logic_Arith_Gt_Uns_Int,
        Self::Ieee_Std_Logic_Arith_Gt_Int_Uns,
        Self::Ieee_Std_Logic_Arith_Gt_Sgn_Int,
        Self::Ieee_Std_Logic_Arith_Gt_Int_Sgn,
        Self::Ieee_Std_Logic_Arith_Ge_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Ge_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Ge_Uns_Sgn,
        Self::Ieee_Std_Logic_Arith_Ge_Sgn_Uns,
        Self::Ieee_Std_Logic_Arith_Ge_Uns_Int,
        Self::Ieee_Std_Logic_Arith_Ge_Int_Uns,
        Self::Ieee_Std_Logic_Arith_Ge_Sgn_Int,
        Self::Ieee_Std_Logic_Arith_Ge_Int_Sgn,
        Self::Ieee_Std_Logic_Arith_Eq_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Eq_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Eq_Uns_Sgn,
        Self::Ieee_Std_Logic_Arith_Eq_Sgn_Uns,
        Self::Ieee_Std_Logic_Arith_Eq_Uns_Int,
        Self::Ieee_Std_Logic_Arith_Eq_Int_Uns,
        Self::Ieee_Std_Logic_Arith_Eq_Sgn_Int,
        Self::Ieee_Std_Logic_Arith_Eq_Int_Sgn,
        Self::Ieee_Std_Logic_Arith_Ne_Uns_Uns,
        Self::Ieee_Std_Logic_Arith_Ne_Sgn_Sgn,
        Self::Ieee_Std_Logic_Arith_Ne_Uns_Sgn,
        Self::Ieee_Std_Logic_Arith_Ne_Sgn_Uns,
        Self::Ieee_Std_Logic_Arith_Ne_Uns_Int,
        Self::Ieee_Std_Logic_Arith_Ne_Int_Uns,
        Self::Ieee_Std_Logic_Arith_Ne_Sgn_Int,
        Self::Ieee_Std_Logic_Arith_Ne_Int_Sgn,
        Self::Ieee_Std_Logic_Misc_And_Reduce_Slv,
        Self::Ieee_Std_Logic_Misc_And_Reduce_Suv,
        Self::Ieee_Std_Logic_Misc_Nand_Reduce_Slv,
        Self::Ieee_Std_Logic_Misc_Nand_Reduce_Suv,
        Self::Ieee_Std_Logic_Misc_Or_Reduce_Slv,
        Self::Ieee_Std_Logic_Misc_Or_Reduce_Suv,
        Self::Ieee_Std_Logic_Misc_Nor_Reduce_Slv,
        Self::Ieee_Std_Logic_Misc_Nor_Reduce_Suv,
        Self::Ieee_Std_Logic_Misc_Xor_Reduce_Slv,
        Self::Ieee_Std_Logic_Misc_Xor_Reduce_Suv,
        Self::Ieee_Std_Logic_Misc_Xnor_Reduce_Slv,
        Self::Ieee_Std_Logic_Misc_Xnor_Reduce_Suv,
    ];

    const IMAGES: [&'static str; 759] = [
        "error",
        "boolean_and",
        "boolean_or",
        "boolean_nand",
        "boolean_nor",
        "boolean_xor",
        "boolean_xnor",
        "boolean_not",
        "boolean_rising_edge",
        "boolean_falling_edge",
        "enum_equality",
        "enum_inequality",
        "enum_less",
        "enum_less_equal",
        "enum_greater",
        "enum_greater_equal",
        "bit_and",
        "bit_or",
        "bit_nand",
        "bit_nor",
        "bit_xor",
        "bit_xnor",
        "bit_not",
        "bit_match_equality",
        "bit_match_inequality",
        "bit_match_less",
        "bit_match_less_equal",
        "bit_match_greater",
        "bit_match_greater_equal",
        "bit_condition",
        "integer_equality",
        "integer_inequality",
        "integer_less",
        "integer_less_equal",
        "integer_greater",
        "integer_greater_equal",
        "integer_identity",
        "integer_negation",
        "integer_absolute",
        "integer_plus",
        "integer_minus",
        "integer_mul",
        "integer_div",
        "integer_mod",
        "integer_rem",
        "integer_exp",
        "floating_equality",
        "floating_inequality",
        "floating_less",
        "floating_less_equal",
        "floating_greater",
        "floating_greater_equal",
        "floating_identity",
        "floating_negation",
        "floating_absolute",
        "floating_plus",
        "floating_minus",
        "floating_mul",
        "floating_div",
        "floating_exp",
        "universal_r_i_mul",
        "universal_i_r_mul",
        "universal_r_i_div",
        "physical_equality",
        "physical_inequality",
        "physical_less",
        "physical_less_equal",
        "physical_greater",
        "physical_greater_equal",
        "physical_identity",
        "physical_negation",
        "physical_absolute",
        "physical_plus",
        "physical_minus",
        "physical_integer_mul",
        "physical_real_mul",
        "integer_physical_mul",
        "real_physical_mul",
        "physical_integer_div",
        "physical_real_div",
        "physical_physical_div",
        "physical_mod",
        "physical_rem",
        "access_equality",
        "access_inequality",
        "record_equality",
        "record_inequality",
        "array_equality",
        "array_inequality",
        "array_less",
        "array_less_equal",
        "array_greater",
        "array_greater_equal",
        "array_array_concat",
        "array_element_concat",
        "element_array_concat",
        "element_element_concat",
        "array_minimum",
        "array_maximum",
        "vector_minimum",
        "vector_maximum",
        "array_sll",
        "array_srl",
        "array_sla",
        "array_sra",
        "array_rol",
        "array_ror",
        "tf_array_and",
        "tf_array_or",
        "tf_array_nand",
        "tf_array_nor",
        "tf_array_xor",
        "tf_array_xnor",
        "tf_array_not",
        "tf_reduction_and",
        "tf_reduction_or",
        "tf_reduction_nand",
        "tf_reduction_nor",
        "tf_reduction_xor",
        "tf_reduction_xnor",
        "tf_reduction_not",
        "tf_array_element_and",
        "tf_element_array_and",
        "tf_array_element_or",
        "tf_element_array_or",
        "tf_array_element_nand",
        "tf_element_array_nand",
        "tf_array_element_nor",
        "tf_element_array_nor",
        "tf_array_element_xor",
        "tf_element_array_xor",
        "tf_array_element_xnor",
        "tf_element_array_xnor",
        "bit_array_match_equality",
        "bit_array_match_inequality",
        "std_ulogic_match_equality",
        "std_ulogic_match_inequality",
        "std_ulogic_match_less",
        "std_ulogic_match_less_equal",
        "std_ulogic_match_greater",
        "std_ulogic_match_greater_equal",
        "std_ulogic_array_match_equality",
        "std_ulogic_array_match_inequality",
        "enum_minimum",
        "enum_maximum",
        "enum_to_string",
        "integer_minimum",
        "integer_maximum",
        "integer_to_string",
        "bit_rising_edge",
        "bit_falling_edge",
        "floating_minimum",
        "floating_maximum",
        "floating_to_string",
        "real_to_string_digits",
        "real_to_string_format",
        "physical_minimum",
        "physical_maximum",
        "physical_to_string",
        "time_to_string_unit",
        "array_char_to_string",
        "bit_vector_to_ostring",
        "bit_vector_to_hstring",
        "deallocate",
        "file_open",
        "file_open_status",
        "file_close",
        "read",
        "read_length",
        "flush",
        "write",
        "endfile",
        "now_function",
        "real_now_function",
        "frequency_function",
        "none",
        "foreign_untruncated_text_read",
        "foreign_textio_read_real",
        "foreign_textio_write_real",
        "std_env_stop_status",
        "std_env_stop",
        "std_env_finish_status",
        "std_env_finish",
        "std_env_resolution_limit",
        "ieee_1164_scalar_and",
        "ieee_1164_scalar_nand",
        "ieee_1164_scalar_or",
        "ieee_1164_scalar_nor",
        "ieee_1164_scalar_xor",
        "ieee_1164_scalar_xnor",
        "ieee_1164_scalar_not",
        "ieee_1164_vector_and",
        "ieee_1164_vector_nand",
        "ieee_1164_vector_or",
        "ieee_1164_vector_nor",
        "ieee_1164_vector_xor",
        "ieee_1164_vector_xnor",
        "ieee_1164_vector_not",
        "ieee_1164_to_bit",
        "ieee_1164_to_bitvector",
        "ieee_1164_to_stdulogic",
        "ieee_1164_to_stdlogicvector_bv",
        "ieee_1164_to_stdlogicvector_suv",
        "ieee_1164_to_stdulogicvector_bv",
        "ieee_1164_to_stdulogicvector_slv",
        "ieee_1164_to_x01_slv",
        "ieee_1164_to_x01_suv",
        "ieee_1164_to_x01_log",
        "ieee_1164_to_x01_bv_slv",
        "ieee_1164_to_x01_bv_suv",
        "ieee_1164_to_x01_bit_log",
        "ieee_1164_to_x01z_slv",
        "ieee_1164_to_x01z_suv",
        "ieee_1164_to_x01z_log",
        "ieee_1164_to_x01z_bv_slv",
        "ieee_1164_to_x01z_bv_suv",
        "ieee_1164_to_x01z_bit_log",
        "ieee_1164_to_ux01_slv",
        "ieee_1164_to_ux01_suv",
        "ieee_1164_to_ux01_log",
        "ieee_1164_to_ux01_bv_slv",
        "ieee_1164_to_ux01_bv_suv",
        "ieee_1164_to_ux01_bit_log",
        "ieee_1164_is_x_slv",
        "ieee_1164_is_x_log",
        "ieee_1164_rising_edge",
        "ieee_1164_falling_edge",
        "ieee_1164_and_suv_log",
        "ieee_1164_and_log_suv",
        "ieee_1164_nand_suv_log",
        "ieee_1164_nand_log_suv",
        "ieee_1164_or_suv_log",
        "ieee_1164_or_log_suv",
        "ieee_1164_nor_suv_log",
        "ieee_1164_nor_log_suv",
        "ieee_1164_xor_suv_log",
        "ieee_1164_xor_log_suv",
        "ieee_1164_xnor_suv_log",
        "ieee_1164_xnor_log_suv",
        "ieee_1164_and_suv",
        "ieee_1164_nand_suv",
        "ieee_1164_or_suv",
        "ieee_1164_nor_suv",
        "ieee_1164_xor_suv",
        "ieee_1164_xnor_suv",
        "ieee_1164_vector_sll",
        "ieee_1164_vector_srl",
        "ieee_1164_vector_rol",
        "ieee_1164_vector_ror",
        "ieee_1164_condition_operator",
        "ieee_1164_to_01_log_log",
        "ieee_1164_to_01_slv_log",
        "ieee_1164_to_hstring",
        "ieee_1164_to_ostring",
        "ieee_numeric_std_toint_uns_nat",
        "ieee_numeric_std_toint_sgn_int",
        "ieee_numeric_std_touns_nat_nat_uns",
        "ieee_numeric_std_touns_nat_uns_uns",
        "ieee_numeric_std_tosgn_int_nat_sgn",
        "ieee_numeric_std_tosgn_int_sgn_sgn",
        "ieee_numeric_std_resize_uns_nat",
        "ieee_numeric_std_resize_sgn_nat",
        "ieee_numeric_std_resize_uns_uns",
        "ieee_numeric_std_resize_sgn_sgn",
        "ieee_numeric_std_add_uns_uns",
        "ieee_numeric_std_add_uns_nat",
        "ieee_numeric_std_add_nat_uns",
        "ieee_numeric_std_add_uns_log",
        "ieee_numeric_std_add_log_uns",
        "ieee_numeric_std_add_sgn_sgn",
        "ieee_numeric_std_add_sgn_int",
        "ieee_numeric_std_add_int_sgn",
        "ieee_numeric_std_add_sgn_log",
        "ieee_numeric_std_add_log_sgn",
        "ieee_numeric_std_sub_uns_uns",
        "ieee_numeric_std_sub_uns_nat",
        "ieee_numeric_std_sub_nat_uns",
        "ieee_numeric_std_sub_uns_log",
        "ieee_numeric_std_sub_log_uns",
        "ieee_numeric_std_sub_sgn_sgn",
        "ieee_numeric_std_sub_sgn_int",
        "ieee_numeric_std_sub_int_sgn",
        "ieee_numeric_std_sub_sgn_log",
        "ieee_numeric_std_sub_log_sgn",
        "ieee_numeric_std_mul_uns_uns",
        "ieee_numeric_std_mul_uns_nat",
        "ieee_numeric_std_mul_nat_uns",
        "ieee_numeric_std_mul_sgn_sgn",
        "ieee_numeric_std_mul_sgn_int",
        "ieee_numeric_std_mul_int_sgn",
        "ieee_numeric_std_div_uns_uns",
        "ieee_numeric_std_div_uns_nat",
        "ieee_numeric_std_div_nat_uns",
        "ieee_numeric_std_div_sgn_sgn",
        "ieee_numeric_std_div_sgn_int",
        "ieee_numeric_std_div_int_sgn",
        "ieee_numeric_std_rem_uns_uns",
        "ieee_numeric_std_rem_uns_nat",
        "ieee_numeric_std_rem_nat_uns",
        "ieee_numeric_std_rem_sgn_sgn",
        "ieee_numeric_std_rem_sgn_int",
        "ieee_numeric_std_rem_int_sgn",
        "ieee_numeric_std_mod_uns_uns",
        "ieee_numeric_std_mod_uns_nat",
        "ieee_numeric_std_mod_nat_uns",
        "ieee_numeric_std_mod_sgn_sgn",
        "ieee_numeric_std_mod_sgn_int",
        "ieee_numeric_std_mod_int_sgn",
        "ieee_numeric_std_gt_uns_uns",
        "ieee_numeric_std_gt_uns_nat",
        "ieee_numeric_std_gt_nat_uns",
        "ieee_numeric_std_gt_sgn_sgn",
        "ieee_numeric_std_gt_sgn_int",
        "ieee_numeric_std_gt_int_sgn",
        "ieee_numeric_std_lt_uns_uns",
        "ieee_numeric_std_lt_uns_nat",
        "ieee_numeric_std_lt_nat_uns",
        "ieee_numeric_std_lt_sgn_sgn",
        "ieee_numeric_std_lt_sgn_int",
        "ieee_numeric_std_lt_int_sgn",
        "ieee_numeric_std_le_uns_uns",
        "ieee_numeric_std_le_uns_nat",
        "ieee_numeric_std_le_nat_uns",
        "ieee_numeric_std_le_sgn_sgn",
        "ieee_numeric_std_le_sgn_int",
        "ieee_numeric_std_le_int_sgn",
        "ieee_numeric_std_ge_uns_uns",
        "ieee_numeric_std_ge_uns_nat",
        "ieee_numeric_std_ge_nat_uns",
        "ieee_numeric_std_ge_sgn_sgn",
        "ieee_numeric_std_ge_sgn_int",
        "ieee_numeric_std_ge_int_sgn",
        "ieee_numeric_std_eq_uns_uns",
        "ieee_numeric_std_eq_uns_nat",
        "ieee_numeric_std_eq_nat_uns",
        "ieee_numeric_std_eq_sgn_sgn",
        "ieee_numeric_std_eq_sgn_int",
        "ieee_numeric_std_eq_int_sgn",
        "ieee_numeric_std_ne_uns_uns",
        "ieee_numeric_std_ne_uns_nat",
        "ieee_numeric_std_ne_nat_uns",
        "ieee_numeric_std_ne_sgn_sgn",
        "ieee_numeric_std_ne_sgn_int",
        "ieee_numeric_std_ne_int_sgn",
        "ieee_numeric_std_match_gt_uns_uns",
        "ieee_numeric_std_match_gt_uns_nat",
        "ieee_numeric_std_match_gt_nat_uns",
        "ieee_numeric_std_match_gt_sgn_sgn",
        "ieee_numeric_std_match_gt_sgn_int",
        "ieee_numeric_std_match_gt_int_sgn",
        "ieee_numeric_std_match_lt_uns_uns",
        "ieee_numeric_std_match_lt_uns_nat",
        "ieee_numeric_std_match_lt_nat_uns",
        "ieee_numeric_std_match_lt_sgn_sgn",
        "ieee_numeric_std_match_lt_sgn_int",
        "ieee_numeric_std_match_lt_int_sgn",
        "ieee_numeric_std_match_le_uns_uns",
        "ieee_numeric_std_match_le_uns_nat",
        "ieee_numeric_std_match_le_nat_uns",
        "ieee_numeric_std_match_le_sgn_sgn",
        "ieee_numeric_std_match_le_sgn_int",
        "ieee_numeric_std_match_le_int_sgn",
        "ieee_numeric_std_match_ge_uns_uns",
        "ieee_numeric_std_match_ge_uns_nat",
        "ieee_numeric_std_match_ge_nat_uns",
        "ieee_numeric_std_match_ge_sgn_sgn",
        "ieee_numeric_std_match_ge_sgn_int",
        "ieee_numeric_std_match_ge_int_sgn",
        "ieee_numeric_std_match_eq_uns_uns",
        "ieee_numeric_std_match_eq_uns_nat",
        "ieee_numeric_std_match_eq_nat_uns",
        "ieee_numeric_std_match_eq_sgn_sgn",
        "ieee_numeric_std_match_eq_sgn_int",
        "ieee_numeric_std_match_eq_int_sgn",
        "ieee_numeric_std_match_ne_uns_uns",
        "ieee_numeric_std_match_ne_uns_nat",
        "ieee_numeric_std_match_ne_nat_uns",
        "ieee_numeric_std_match_ne_sgn_sgn",
        "ieee_numeric_std_match_ne_sgn_int",
        "ieee_numeric_std_match_ne_int_sgn",
        "ieee_numeric_std_sll_uns_int",
        "ieee_numeric_std_sll_sgn_int",
        "ieee_numeric_std_srl_uns_int",
        "ieee_numeric_std_srl_sgn_int",
        "ieee_numeric_std_sla_uns_int",
        "ieee_numeric_std_sla_sgn_int",
        "ieee_numeric_std_sra_uns_int",
        "ieee_numeric_std_sra_sgn_int",
        "ieee_numeric_std_rol_uns_int",
        "ieee_numeric_std_rol_sgn_int",
        "ieee_numeric_std_ror_uns_int",
        "ieee_numeric_std_ror_sgn_int",
        "ieee_numeric_std_and_uns_uns",
        "ieee_numeric_std_and_uns_log",
        "ieee_numeric_std_and_log_uns",
        "ieee_numeric_std_and_sgn_sgn",
        "ieee_numeric_std_and_sgn_log",
        "ieee_numeric_std_and_log_sgn",
        "ieee_numeric_std_nand_uns_uns",
        "ieee_numeric_std_nand_uns_log",
        "ieee_numeric_std_nand_log_uns",
        "ieee_numeric_std_nand_sgn_sgn",
        "ieee_numeric_std_nand_sgn_log",
        "ieee_numeric_std_nand_log_sgn",
        "ieee_numeric_std_or_uns_uns",
        "ieee_numeric_std_or_uns_log",
        "ieee_numeric_std_or_log_uns",
        "ieee_numeric_std_or_sgn_sgn",
        "ieee_numeric_std_or_sgn_log",
        "ieee_numeric_std_or_log_sgn",
        "ieee_numeric_std_nor_uns_uns",
        "ieee_numeric_std_nor_uns_log",
        "ieee_numeric_std_nor_log_uns",
        "ieee_numeric_std_nor_sgn_sgn",
        "ieee_numeric_std_nor_sgn_log",
        "ieee_numeric_std_nor_log_sgn",
        "ieee_numeric_std_xor_uns_uns",
        "ieee_numeric_std_xor_uns_log",
        "ieee_numeric_std_xor_log_uns",
        "ieee_numeric_std_xor_sgn_sgn",
        "ieee_numeric_std_xor_sgn_log",
        "ieee_numeric_std_xor_log_sgn",
        "ieee_numeric_std_xnor_uns_uns",
        "ieee_numeric_std_xnor_uns_log",
        "ieee_numeric_std_xnor_log_uns",
        "ieee_numeric_std_xnor_sgn_sgn",
        "ieee_numeric_std_xnor_sgn_log",
        "ieee_numeric_std_xnor_log_sgn",
        "ieee_numeric_std_not_uns",
        "ieee_numeric_std_not_sgn",
        "ieee_numeric_std_abs_sgn",
        "ieee_numeric_std_neg_uns",
        "ieee_numeric_std_neg_sgn",
        "ieee_numeric_std_min_uns_uns",
        "ieee_numeric_std_min_uns_nat",
        "ieee_numeric_std_min_nat_uns",
        "ieee_numeric_std_min_sgn_sgn",
        "ieee_numeric_std_min_sgn_int",
        "ieee_numeric_std_min_int_sgn",
        "ieee_numeric_std_max_uns_uns",
        "ieee_numeric_std_max_uns_nat",
        "ieee_numeric_std_max_nat_uns",
        "ieee_numeric_std_max_sgn_sgn",
        "ieee_numeric_std_max_sgn_int",
        "ieee_numeric_std_max_int_sgn",
        "ieee_numeric_std_shf_left_uns_nat",
        "ieee_numeric_std_shf_right_uns_nat",
        "ieee_numeric_std_shf_left_sgn_nat",
        "ieee_numeric_std_shf_right_sgn_nat",
        "ieee_numeric_std_rot_left_uns_nat",
        "ieee_numeric_std_rot_right_uns_nat",
        "ieee_numeric_std_rot_left_sgn_nat",
        "ieee_numeric_std_rot_right_sgn_nat",
        "ieee_numeric_std_and_sgn",
        "ieee_numeric_std_nand_sgn",
        "ieee_numeric_std_or_sgn",
        "ieee_numeric_std_nor_sgn",
        "ieee_numeric_std_xor_sgn",
        "ieee_numeric_std_xnor_sgn",
        "ieee_numeric_std_and_uns",
        "ieee_numeric_std_nand_uns",
        "ieee_numeric_std_or_uns",
        "ieee_numeric_std_nor_uns",
        "ieee_numeric_std_xor_uns",
        "ieee_numeric_std_xnor_uns",
        "ieee_numeric_std_find_leftmost_uns",
        "ieee_numeric_std_find_rightmost_uns",
        "ieee_numeric_std_find_leftmost_sgn",
        "ieee_numeric_std_find_rightmost_sgn",
        "ieee_numeric_std_match_log",
        "ieee_numeric_std_match_uns",
        "ieee_numeric_std_match_sgn",
        "ieee_numeric_std_match_slv",
        "ieee_numeric_std_match_suv",
        "ieee_numeric_std_to_01_uns",
        "ieee_numeric_std_to_01_sgn",
        "ieee_numeric_std_to_x01_uns",
        "ieee_numeric_std_to_x01_sgn",
        "ieee_numeric_std_to_x01z_uns",
        "ieee_numeric_std_to_x01z_sgn",
        "ieee_numeric_std_to_ux01_uns",
        "ieee_numeric_std_to_ux01_sgn",
        "ieee_numeric_std_is_x_uns",
        "ieee_numeric_std_is_x_sgn",
        "ieee_numeric_std_to_hstring_uns",
        "ieee_numeric_std_to_ostring_uns",
        "ieee_numeric_std_to_hstring_sgn",
        "ieee_numeric_std_to_ostring_sgn",
        "ieee_numeric_bit_toint_uns_nat",
        "ieee_numeric_bit_toint_sgn_int",
        "ieee_numeric_bit_touns_nat_nat_uns",
        "ieee_numeric_bit_touns_nat_uns_uns",
        "ieee_numeric_bit_tosgn_int_nat_sgn",
        "ieee_numeric_bit_tosgn_int_sgn_sgn",
        "ieee_numeric_std_unsigned_add_slv_slv",
        "ieee_numeric_std_unsigned_add_slv_nat",
        "ieee_numeric_std_unsigned_add_nat_slv",
        "ieee_numeric_std_unsigned_sub_slv_slv",
        "ieee_numeric_std_unsigned_sub_slv_nat",
        "ieee_numeric_std_unsigned_sub_nat_slv",
        "ieee_numeric_std_unsigned_find_rightmost",
        "ieee_numeric_std_unsigned_find_leftmost",
        "ieee_numeric_std_unsigned_shift_left",
        "ieee_numeric_std_unsigned_shift_right",
        "ieee_numeric_std_unsigned_rotate_left",
        "ieee_numeric_std_unsigned_rotate_right",
        "ieee_numeric_std_unsigned_to_integer_slv_nat",
        "ieee_numeric_std_unsigned_to_slv_nat_nat",
        "ieee_numeric_std_unsigned_to_slv_nat_slv",
        "ieee_numeric_std_unsigned_to_suv_nat_nat",
        "ieee_numeric_std_unsigned_to_suv_nat_suv",
        "ieee_numeric_std_unsigned_resize_slv_nat",
        "ieee_numeric_std_unsigned_resize_slv_slv",
        "ieee_numeric_std_unsigned_maximum_slv_slv",
        "ieee_numeric_std_unsigned_minimum_slv_slv",
        "ieee_math_real_sign",
        "ieee_math_real_ceil",
        "ieee_math_real_floor",
        "ieee_math_real_round",
        "ieee_math_real_trunc",
        "ieee_math_real_mod",
        "ieee_math_real_realmax",
        "ieee_math_real_realmin",
        "ieee_math_real_sqrt",
        "ieee_math_real_cbrt",
        "ieee_math_real_pow_int_real",
        "ieee_math_real_pow_real_real",
        "ieee_math_real_exp",
        "ieee_math_real_log",
        "ieee_math_real_log2",
        "ieee_math_real_log10",
        "ieee_math_real_log_real_real",
        "ieee_math_real_sin",
        "ieee_math_real_cos",
        "ieee_math_real_tan",
        "ieee_math_real_arcsin",
        "ieee_math_real_arccos",
        "ieee_math_real_arctan",
        "ieee_math_real_arctan_real_real",
        "ieee_math_real_sinh",
        "ieee_math_real_cosh",
        "ieee_math_real_tanh",
        "ieee_math_real_arcsinh",
        "ieee_math_real_arccosh",
        "ieee_math_real_arctanh",
        "ieee_std_logic_unsigned_add_slv_slv",
        "ieee_std_logic_unsigned_add_slv_int",
        "ieee_std_logic_unsigned_add_int_slv",
        "ieee_std_logic_unsigned_add_slv_log",
        "ieee_std_logic_unsigned_add_log_slv",
        "ieee_std_logic_unsigned_sub_slv_slv",
        "ieee_std_logic_unsigned_sub_slv_int",
        "ieee_std_logic_unsigned_sub_int_slv",
        "ieee_std_logic_unsigned_sub_slv_log",
        "ieee_std_logic_unsigned_sub_log_slv",
        "ieee_std_logic_unsigned_id_slv",
        "ieee_std_logic_unsigned_mul_slv_slv",
        "ieee_std_logic_unsigned_lt_slv_slv",
        "ieee_std_logic_unsigned_lt_slv_int",
        "ieee_std_logic_unsigned_lt_int_slv",
        "ieee_std_logic_unsigned_le_slv_slv",
        "ieee_std_logic_unsigned_le_slv_int",
        "ieee_std_logic_unsigned_le_int_slv",
        "ieee_std_logic_unsigned_gt_slv_slv",
        "ieee_std_logic_unsigned_gt_slv_int",
        "ieee_std_logic_unsigned_gt_int_slv",
        "ieee_std_logic_unsigned_ge_slv_slv",
        "ieee_std_logic_unsigned_ge_slv_int",
        "ieee_std_logic_unsigned_ge_int_slv",
        "ieee_std_logic_unsigned_eq_slv_slv",
        "ieee_std_logic_unsigned_eq_slv_int",
        "ieee_std_logic_unsigned_eq_int_slv",
        "ieee_std_logic_unsigned_ne_slv_slv",
        "ieee_std_logic_unsigned_ne_slv_int",
        "ieee_std_logic_unsigned_ne_int_slv",
        "ieee_std_logic_unsigned_conv_integer",
        "ieee_std_logic_unsigned_shl",
        "ieee_std_logic_unsigned_shr",
        "ieee_std_logic_signed_add_slv_slv",
        "ieee_std_logic_signed_add_slv_int",
        "ieee_std_logic_signed_add_int_slv",
        "ieee_std_logic_signed_add_slv_log",
        "ieee_std_logic_signed_add_log_slv",
        "ieee_std_logic_signed_sub_slv_slv",
        "ieee_std_logic_signed_sub_slv_int",
        "ieee_std_logic_signed_sub_int_slv",
        "ieee_std_logic_signed_sub_slv_log",
        "ieee_std_logic_signed_sub_log_slv",
        "ieee_std_logic_signed_id_slv",
        "ieee_std_logic_signed_neg_slv",
        "ieee_std_logic_signed_abs_slv",
        "ieee_std_logic_signed_mul_slv_slv",
        "ieee_std_logic_signed_lt_slv_slv",
        "ieee_std_logic_signed_lt_slv_int",
        "ieee_std_logic_signed_lt_int_slv",
        "ieee_std_logic_signed_le_slv_slv",
        "ieee_std_logic_signed_le_slv_int",
        "ieee_std_logic_signed_le_int_slv",
        "ieee_std_logic_signed_gt_slv_slv",
        "ieee_std_logic_signed_gt_slv_int",
        "ieee_std_logic_signed_gt_int_slv",
        "ieee_std_logic_signed_ge_slv_slv",
        "ieee_std_logic_signed_ge_slv_int",
        "ieee_std_logic_signed_ge_int_slv",
        "ieee_std_logic_signed_eq_slv_slv",
        "ieee_std_logic_signed_eq_slv_int",
        "ieee_std_logic_signed_eq_int_slv",
        "ieee_std_logic_signed_ne_slv_slv",
        "ieee_std_logic_signed_ne_slv_int",
        "ieee_std_logic_signed_ne_int_slv",
        "ieee_std_logic_signed_conv_integer",
        "ieee_std_logic_signed_shl",
        "ieee_std_logic_signed_shr",
        "ieee_std_logic_arith_conv_unsigned_int",
        "ieee_std_logic_arith_conv_unsigned_uns",
        "ieee_std_logic_arith_conv_unsigned_sgn",
        "ieee_std_logic_arith_conv_unsigned_log",
        "ieee_std_logic_arith_conv_signed_int",
        "ieee_std_logic_arith_conv_signed_uns",
        "ieee_std_logic_arith_conv_signed_sgn",
        "ieee_std_logic_arith_conv_signed_log",
        "ieee_std_logic_arith_conv_integer_int",
        "ieee_std_logic_arith_conv_integer_uns",
        "ieee_std_logic_arith_conv_integer_sgn",
        "ieee_std_logic_arith_conv_integer_log",
        "ieee_std_logic_arith_conv_vector_int",
        "ieee_std_logic_arith_conv_vector_uns",
        "ieee_std_logic_arith_conv_vector_sgn",
        "ieee_std_logic_arith_conv_vector_log",
        "ieee_std_logic_arith_ext",
        "ieee_std_logic_arith_sxt",
        "ieee_std_logic_arith_id_uns_uns",
        "ieee_std_logic_arith_id_sgn_sgn",
        "ieee_std_logic_arith_neg_sgn_sgn",
        "ieee_std_logic_arith_abs_sgn_sgn",
        "ieee_std_logic_arith_shl_uns",
        "ieee_std_logic_arith_shl_sgn",
        "ieee_std_logic_arith_shr_uns",
        "ieee_std_logic_arith_shr_sgn",
        "ieee_std_logic_arith_id_uns_slv",
        "ieee_std_logic_arith_id_sgn_slv",
        "ieee_std_logic_arith_neg_sgn_slv",
        "ieee_std_logic_arith_abs_sgn_slv",
        "ieee_std_logic_arith_mul_uns_uns_uns",
        "ieee_std_logic_arith_mul_sgn_sgn_sgn",
        "ieee_std_logic_arith_mul_sgn_uns_sgn",
        "ieee_std_logic_arith_mul_uns_sgn_sgn",
        "ieee_std_logic_arith_mul_uns_uns_slv",
        "ieee_std_logic_arith_mul_sgn_sgn_slv",
        "ieee_std_logic_arith_mul_sgn_uns_slv",
        "ieee_std_logic_arith_mul_uns_sgn_slv",
        "ieee_std_logic_arith_add_uns_uns_uns",
        "ieee_std_logic_arith_add_sgn_sgn_sgn",
        "ieee_std_logic_arith_add_uns_sgn_sgn",
        "ieee_std_logic_arith_add_sgn_uns_sgn",
        "ieee_std_logic_arith_add_uns_int_uns",
        "ieee_std_logic_arith_add_int_uns_uns",
        "ieee_std_logic_arith_add_sgn_int_sgn",
        "ieee_std_logic_arith_add_int_sgn_sgn",
        "ieee_std_logic_arith_add_uns_log_uns",
        "ieee_std_logic_arith_add_log_uns_uns",
        "ieee_std_logic_arith_add_sgn_log_sgn",
        "ieee_std_logic_arith_add_log_sgn_sgn",
        "ieee_std_logic_arith_add_uns_uns_slv",
        "ieee_std_logic_arith_add_sgn_sgn_slv",
        "ieee_std_logic_arith_add_uns_sgn_slv",
        "ieee_std_logic_arith_add_sgn_uns_slv",
        "ieee_std_logic_arith_add_uns_int_slv",
        "ieee_std_logic_arith_add_int_uns_slv",
        "ieee_std_logic_arith_add_sgn_int_slv",
        "ieee_std_logic_arith_add_int_sgn_slv",
        "ieee_std_logic_arith_add_uns_log_slv",
        "ieee_std_logic_arith_add_log_uns_slv",
        "ieee_std_logic_arith_add_sgn_log_slv",
        "ieee_std_logic_arith_add_log_sgn_slv",
        "ieee_std_logic_arith_sub_uns_uns_uns",
        "ieee_std_logic_arith_sub_sgn_sgn_sgn",
        "ieee_std_logic_arith_sub_uns_sgn_sgn",
        "ieee_std_logic_arith_sub_sgn_uns_sgn",
        "ieee_std_logic_arith_sub_uns_int_uns",
        "ieee_std_logic_arith_sub_int_uns_uns",
        "ieee_std_logic_arith_sub_sgn_int_sgn",
        "ieee_std_logic_arith_sub_int_sgn_sgn",
        "ieee_std_logic_arith_sub_uns_log_uns",
        "ieee_std_logic_arith_sub_log_uns_uns",
        "ieee_std_logic_arith_sub_sgn_log_sgn",
        "ieee_std_logic_arith_sub_log_sgn_sgn",
        "ieee_std_logic_arith_sub_uns_uns_slv",
        "ieee_std_logic_arith_sub_sgn_sgn_slv",
        "ieee_std_logic_arith_sub_uns_sgn_slv",
        "ieee_std_logic_arith_sub_sgn_uns_slv",
        "ieee_std_logic_arith_sub_uns_int_slv",
        "ieee_std_logic_arith_sub_int_uns_slv",
        "ieee_std_logic_arith_sub_sgn_int_slv",
        "ieee_std_logic_arith_sub_int_sgn_slv",
        "ieee_std_logic_arith_sub_uns_log_slv",
        "ieee_std_logic_arith_sub_log_uns_slv",
        "ieee_std_logic_arith_sub_sgn_log_slv",
        "ieee_std_logic_arith_sub_log_sgn_slv",
        "ieee_std_logic_arith_lt_uns_uns",
        "ieee_std_logic_arith_lt_sgn_sgn",
        "ieee_std_logic_arith_lt_uns_sgn",
        "ieee_std_logic_arith_lt_sgn_uns",
        "ieee_std_logic_arith_lt_uns_int",
        "ieee_std_logic_arith_lt_int_uns",
        "ieee_std_logic_arith_lt_sgn_int",
        "ieee_std_logic_arith_lt_int_sgn",
        "ieee_std_logic_arith_le_uns_uns",
        "ieee_std_logic_arith_le_sgn_sgn",
        "ieee_std_logic_arith_le_uns_sgn",
        "ieee_std_logic_arith_le_sgn_uns",
        "ieee_std_logic_arith_le_uns_int",
        "ieee_std_logic_arith_le_int_uns",
        "ieee_std_logic_arith_le_sgn_int",
        "ieee_std_logic_arith_le_int_sgn",
        "ieee_std_logic_arith_gt_uns_uns",
        "ieee_std_logic_arith_gt_sgn_sgn",
        "ieee_std_logic_arith_gt_uns_sgn",
        "ieee_std_logic_arith_gt_sgn_uns",
        "ieee_std_logic_arith_gt_uns_int",
        "ieee_std_logic_arith_gt_int_uns",
        "ieee_std_logic_arith_gt_sgn_int",
        "ieee_std_logic_arith_gt_int_sgn",
        "ieee_std_logic_arith_ge_uns_uns",
        "ieee_std_logic_arith_ge_sgn_sgn",
        "ieee_std_logic_arith_ge_uns_sgn",
        "ieee_std_logic_arith_ge_sgn_uns",
        "ieee_std_logic_arith_ge_uns_int",
        "ieee_std_logic_arith_ge_int_uns",
        "ieee_std_logic_arith_ge_sgn_int",
        "ieee_std_logic_arith_ge_int_sgn",
        "ieee_std_logic_arith_eq_uns_uns",
        "ieee_std_logic_arith_eq_sgn_sgn",
        "ieee_std_logic_arith_eq_uns_sgn",
        "ieee_std_logic_arith_eq_sgn_uns",
        "ieee_std_logic_arith_eq_uns_int",
        "ieee_std_logic_arith_eq_int_uns",
        "ieee_std_logic_arith_eq_sgn_int",
        "ieee_std_logic_arith_eq_int_sgn",
        "ieee_std_logic_arith_ne_uns_uns",
        "ieee_std_logic_arith_ne_sgn_sgn",
        "ieee_std_logic_arith_ne_uns_sgn",
        "ieee_std_logic_arith_ne_sgn_uns",
        "ieee_std_logic_arith_ne_uns_int",
        "ieee_std_logic_arith_ne_int_uns",
        "ieee_std_logic_arith_ne_sgn_int",
        "ieee_std_logic_arith_ne_int_sgn",
        "ieee_std_logic_misc_and_reduce_slv",
        "ieee_std_logic_misc_and_reduce_suv",
        "ieee_std_logic_misc_nand_reduce_slv",
        "ieee_std_logic_misc_nand_reduce_suv",
        "ieee_std_logic_misc_or_reduce_slv",
        "ieee_std_logic_misc_or_reduce_suv",
        "ieee_std_logic_misc_nor_reduce_slv",
        "ieee_std_logic_misc_nor_reduce_suv",
        "ieee_std_logic_misc_xor_reduce_slv",
        "ieee_std_logic_misc_xor_reduce_suv",
        "ieee_std_logic_misc_xnor_reduce_slv",
        "ieee_std_logic_misc_xnor_reduce_suv",
    ];
}
type Iir = u32;
type FileChecksumId = u32;
type TimeStampId = u32;
type SourceFileEntry = u32;
type DateType = u32;
type NameId = u32;
type SourcePtr = u32;
type String8Id = u32;
type PSLNode = u32;
type PSLNFA = u32;
type Tok = u8;
pub enum TriStateType {
   Unknown,
   False,
   True,
}
pub enum DirectionType {
   To,
   Downto,
}

extern "C" {
  #[link_name = "vhdl__nodes__get_kind"]
  fn get_kind(n: u32) -> Kind;

  #[link_name = "vhdl__nodes__get_location"]
  fn get_location(n: u32) -> u32;

  #[link_name = "vhdl__nodes__get_first_design_unit"]
  fn get_first_design_unit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_first_design_unit"]
  fn set_first_design_unit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_last_design_unit"]
  fn get_last_design_unit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_last_design_unit"]
  fn set_last_design_unit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_library_declaration"]
  fn get_library_declaration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_library_declaration"]
  fn set_library_declaration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_file_checksum"]
  fn get_file_checksum(n: u32) -> FileChecksumId;

  #[link_name = "vhdl__nodes__set_file_checksum"]
  fn set_file_checksum(n: u32, v: FileChecksumId);

  #[link_name = "vhdl__nodes__get_analysis_time_stamp"]
  fn get_analysis_time_stamp(n: u32) -> TimeStampId;

  #[link_name = "vhdl__nodes__set_analysis_time_stamp"]
  fn set_analysis_time_stamp(n: u32, v: TimeStampId);

  #[link_name = "vhdl__nodes__get_design_file_source"]
  fn get_design_file_source(n: u32) -> SourceFileEntry;

  #[link_name = "vhdl__nodes__set_design_file_source"]
  fn set_design_file_source(n: u32, v: SourceFileEntry);

  #[link_name = "vhdl__nodes__get_library"]
  fn get_library(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_library"]
  fn set_library(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_design_file_filename"]
  fn get_design_file_filename(n: u32) -> NameId;

  #[link_name = "vhdl__nodes__set_design_file_filename"]
  fn set_design_file_filename(n: u32, v: NameId);

  #[link_name = "vhdl__nodes__get_design_file_directory"]
  fn get_design_file_directory(n: u32) -> NameId;

  #[link_name = "vhdl__nodes__set_design_file_directory"]
  fn set_design_file_directory(n: u32, v: NameId);

  #[link_name = "vhdl__nodes__get_design_file"]
  fn get_design_file(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_design_file"]
  fn set_design_file(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_design_file_chain"]
  fn get_design_file_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_design_file_chain"]
  fn set_design_file_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_library_directory"]
  fn get_library_directory(n: u32) -> NameId;

  #[link_name = "vhdl__nodes__set_library_directory"]
  fn set_library_directory(n: u32, v: NameId);

  #[link_name = "vhdl__nodes__get_date"]
  fn get_date(n: u32) -> DateType;

  #[link_name = "vhdl__nodes__set_date"]
  fn set_date(n: u32, v: DateType);

  #[link_name = "vhdl__nodes__get_context_items"]
  fn get_context_items(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_context_items"]
  fn set_context_items(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_dependence_list"]
  fn get_dependence_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_dependence_list"]
  fn set_dependence_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_analysis_checks_list"]
  fn get_analysis_checks_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_analysis_checks_list"]
  fn set_analysis_checks_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_date_state"]
  fn get_date_state(n: u32) -> DateStateType;

  #[link_name = "vhdl__nodes__set_date_state"]
  fn set_date_state(n: u32, v: DateStateType);

  #[link_name = "vhdl__nodes__get_guarded_target_state"]
  fn get_guarded_target_state(n: u32) -> TriStateType;

  #[link_name = "vhdl__nodes__set_guarded_target_state"]
  fn set_guarded_target_state(n: u32, v: TriStateType);

  #[link_name = "vhdl__nodes__get_library_unit"]
  fn get_library_unit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_library_unit"]
  fn set_library_unit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_hash_chain"]
  fn get_hash_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_hash_chain"]
  fn set_hash_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_design_unit_source_pos"]
  fn get_design_unit_source_pos(n: u32) -> SourcePtr;

  #[link_name = "vhdl__nodes__set_design_unit_source_pos"]
  fn set_design_unit_source_pos(n: u32, v: SourcePtr);

  #[link_name = "vhdl__nodes__get_design_unit_source_line"]
  fn get_design_unit_source_line(n: u32) -> i32;

  #[link_name = "vhdl__nodes__set_design_unit_source_line"]
  fn set_design_unit_source_line(n: u32, v: i32);

  #[link_name = "vhdl__nodes__get_design_unit_source_col"]
  fn get_design_unit_source_col(n: u32) -> i32;

  #[link_name = "vhdl__nodes__set_design_unit_source_col"]
  fn set_design_unit_source_col(n: u32, v: i32);

  #[link_name = "vhdl__nodes__get_value"]
  fn get_value(n: u32) -> i64;

  #[link_name = "vhdl__nodes__set_value"]
  fn set_value(n: u32, v: i64);

  #[link_name = "vhdl__nodes__get_enum_pos"]
  fn get_enum_pos(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_enum_pos"]
  fn set_enum_pos(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_physical_literal"]
  fn get_physical_literal(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_physical_literal"]
  fn set_physical_literal(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_fp_value"]
  fn get_fp_value(n: u32) -> f64;

  #[link_name = "vhdl__nodes__set_fp_value"]
  fn set_fp_value(n: u32, v: f64);

  #[link_name = "vhdl__nodes__get_simple_aggregate_list"]
  fn get_simple_aggregate_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_simple_aggregate_list"]
  fn set_simple_aggregate_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_string8_id"]
  fn get_string8_id(n: u32) -> String8Id;

  #[link_name = "vhdl__nodes__set_string8_id"]
  fn set_string8_id(n: u32, v: String8Id);

  #[link_name = "vhdl__nodes__get_string_length"]
  fn get_string_length(n: u32) -> i32;

  #[link_name = "vhdl__nodes__set_string_length"]
  fn set_string_length(n: u32, v: i32);

  #[link_name = "vhdl__nodes__get_bit_string_base"]
  fn get_bit_string_base(n: u32) -> NumberBaseType;

  #[link_name = "vhdl__nodes__set_bit_string_base"]
  fn set_bit_string_base(n: u32, v: NumberBaseType);

  #[link_name = "vhdl__nodes__get_has_signed"]
  fn get_has_signed(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_signed"]
  fn set_has_signed(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_sign"]
  fn get_has_sign(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_sign"]
  fn set_has_sign(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_length"]
  fn get_has_length(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_length"]
  fn set_has_length(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_literal_length"]
  fn get_literal_length(n: u32) -> i32;

  #[link_name = "vhdl__nodes__set_literal_length"]
  fn set_literal_length(n: u32, v: i32);

  #[link_name = "vhdl__nodes__get_literal_origin"]
  fn get_literal_origin(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_literal_origin"]
  fn set_literal_origin(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_range_origin"]
  fn get_range_origin(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_range_origin"]
  fn set_range_origin(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_literal_subtype"]
  fn get_literal_subtype(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_literal_subtype"]
  fn set_literal_subtype(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_allocator_subtype"]
  fn get_allocator_subtype(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_allocator_subtype"]
  fn set_allocator_subtype(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_entity_class"]
  fn get_entity_class(n: u32) -> Tok;

  #[link_name = "vhdl__nodes__set_entity_class"]
  fn set_entity_class(n: u32, v: Tok);

  #[link_name = "vhdl__nodes__get_entity_name_list"]
  fn get_entity_name_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_entity_name_list"]
  fn set_entity_name_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_attribute_designator"]
  fn get_attribute_designator(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_attribute_designator"]
  fn set_attribute_designator(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_attribute_specification_chain"]
  fn get_attribute_specification_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_attribute_specification_chain"]
  fn set_attribute_specification_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_attribute_specification"]
  fn get_attribute_specification(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_attribute_specification"]
  fn set_attribute_specification(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_static_attribute_flag"]
  fn get_static_attribute_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_static_attribute_flag"]
  fn set_static_attribute_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_signal_list"]
  fn get_signal_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_signal_list"]
  fn set_signal_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_quantity_list"]
  fn get_quantity_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_quantity_list"]
  fn set_quantity_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_designated_entity"]
  fn get_designated_entity(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_designated_entity"]
  fn set_designated_entity(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_formal"]
  fn get_formal(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_formal"]
  fn set_formal(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_actual"]
  fn get_actual(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_actual"]
  fn set_actual(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_open_actual"]
  fn get_open_actual(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_open_actual"]
  fn set_open_actual(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_actual_conversion"]
  fn get_actual_conversion(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_actual_conversion"]
  fn set_actual_conversion(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_formal_conversion"]
  fn get_formal_conversion(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_formal_conversion"]
  fn set_formal_conversion(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_whole_association_flag"]
  fn get_whole_association_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_whole_association_flag"]
  fn set_whole_association_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_collapse_signal_flag"]
  fn get_collapse_signal_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_collapse_signal_flag"]
  fn set_collapse_signal_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_artificial_flag"]
  fn get_artificial_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_artificial_flag"]
  fn set_artificial_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_open_flag"]
  fn get_open_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_open_flag"]
  fn set_open_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_after_drivers_flag"]
  fn get_after_drivers_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_after_drivers_flag"]
  fn set_after_drivers_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_we_value"]
  fn get_we_value(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_we_value"]
  fn set_we_value(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_time"]
  fn get_time(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_time"]
  fn set_time(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_associated_expr"]
  fn get_associated_expr(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_associated_expr"]
  fn set_associated_expr(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_associated_block"]
  fn get_associated_block(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_associated_block"]
  fn set_associated_block(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_associated_chain"]
  fn get_associated_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_associated_chain"]
  fn set_associated_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_choice_name"]
  fn get_choice_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_choice_name"]
  fn set_choice_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_choice_expression"]
  fn get_choice_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_choice_expression"]
  fn set_choice_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_choice_range"]
  fn get_choice_range(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_choice_range"]
  fn set_choice_range(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_same_alternative_flag"]
  fn get_same_alternative_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_same_alternative_flag"]
  fn set_same_alternative_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_element_type_flag"]
  fn get_element_type_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_element_type_flag"]
  fn set_element_type_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_architecture"]
  fn get_architecture(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_architecture"]
  fn set_architecture(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_block_specification"]
  fn get_block_specification(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_block_specification"]
  fn set_block_specification(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_prev_block_configuration"]
  fn get_prev_block_configuration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_prev_block_configuration"]
  fn set_prev_block_configuration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_configuration_item_chain"]
  fn get_configuration_item_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_configuration_item_chain"]
  fn set_configuration_item_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_attribute_value_chain"]
  fn get_attribute_value_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_attribute_value_chain"]
  fn set_attribute_value_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_spec_chain"]
  fn get_spec_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_spec_chain"]
  fn set_spec_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_value_chain"]
  fn get_value_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_value_chain"]
  fn set_value_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_attribute_value_spec_chain"]
  fn get_attribute_value_spec_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_attribute_value_spec_chain"]
  fn set_attribute_value_spec_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_entity_name"]
  fn get_entity_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_entity_name"]
  fn set_entity_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_package"]
  fn get_package(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_package"]
  fn set_package(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_package_body"]
  fn get_package_body(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_package_body"]
  fn set_package_body(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_instance_package_body"]
  fn get_instance_package_body(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_instance_package_body"]
  fn set_instance_package_body(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_owned_instance_package_body"]
  fn get_owned_instance_package_body(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_owned_instance_package_body"]
  fn set_owned_instance_package_body(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_need_body"]
  fn get_need_body(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_need_body"]
  fn set_need_body(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_immediate_body_flag"]
  fn get_immediate_body_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_immediate_body_flag"]
  fn set_immediate_body_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_macro_expand_flag"]
  fn get_macro_expand_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_macro_expand_flag"]
  fn set_macro_expand_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_need_instance_bodies"]
  fn get_need_instance_bodies(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_need_instance_bodies"]
  fn set_need_instance_bodies(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_hierarchical_name"]
  fn get_hierarchical_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_hierarchical_name"]
  fn set_hierarchical_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_vunit_item_chain"]
  fn get_vunit_item_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_vunit_item_chain"]
  fn set_vunit_item_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_bound_vunit_chain"]
  fn get_bound_vunit_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_bound_vunit_chain"]
  fn set_bound_vunit_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_verification_block_configuration"]
  fn get_verification_block_configuration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_verification_block_configuration"]
  fn set_verification_block_configuration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_block_configuration"]
  fn get_block_configuration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_block_configuration"]
  fn set_block_configuration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_concurrent_statement_chain"]
  fn get_concurrent_statement_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_concurrent_statement_chain"]
  fn set_concurrent_statement_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_chain"]
  fn get_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_chain"]
  fn set_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_port_chain"]
  fn get_port_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_port_chain"]
  fn set_port_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_generic_chain"]
  fn get_generic_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_generic_chain"]
  fn set_generic_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_type"]
  fn get_type(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_type"]
  fn set_type(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subtype_indication"]
  fn get_subtype_indication(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subtype_indication"]
  fn set_subtype_indication(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_discrete_range"]
  fn get_discrete_range(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_discrete_range"]
  fn set_discrete_range(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_type_definition"]
  fn get_type_definition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_type_definition"]
  fn set_type_definition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subtype_definition"]
  fn get_subtype_definition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subtype_definition"]
  fn set_subtype_definition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_incomplete_type_declaration"]
  fn get_incomplete_type_declaration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_incomplete_type_declaration"]
  fn set_incomplete_type_declaration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_interface_type_subprograms"]
  fn get_interface_type_subprograms(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_interface_type_subprograms"]
  fn set_interface_type_subprograms(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_interface_type_definition"]
  fn get_interface_type_definition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_interface_type_definition"]
  fn set_interface_type_definition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_nature_definition"]
  fn get_nature_definition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_nature_definition"]
  fn set_nature_definition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_nature"]
  fn get_nature(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_nature"]
  fn set_nature(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subnature_indication"]
  fn get_subnature_indication(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subnature_indication"]
  fn set_subnature_indication(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_reference_terminal_flag"]
  fn get_reference_terminal_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_reference_terminal_flag"]
  fn set_reference_terminal_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_mode"]
  fn get_mode(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_mode"]
  fn set_mode(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_guarded_signal_flag"]
  fn get_guarded_signal_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_guarded_signal_flag"]
  fn set_guarded_signal_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_signal_kind"]
  fn get_signal_kind(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_signal_kind"]
  fn set_signal_kind(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_base_name"]
  fn get_base_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_base_name"]
  fn set_base_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_interface_declaration_chain"]
  fn get_interface_declaration_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_interface_declaration_chain"]
  fn set_interface_declaration_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_default_subprogram"]
  fn get_default_subprogram(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_default_subprogram"]
  fn set_default_subprogram(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_associated_subprogram"]
  fn get_associated_subprogram(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_associated_subprogram"]
  fn set_associated_subprogram(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subprogram_specification"]
  fn get_subprogram_specification(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subprogram_specification"]
  fn set_subprogram_specification(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_sequential_statement_chain"]
  fn get_sequential_statement_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_sequential_statement_chain"]
  fn set_sequential_statement_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_simultaneous_statement_chain"]
  fn get_simultaneous_statement_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_simultaneous_statement_chain"]
  fn set_simultaneous_statement_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subprogram_body"]
  fn get_subprogram_body(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subprogram_body"]
  fn set_subprogram_body(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_overload_number"]
  fn get_overload_number(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_overload_number"]
  fn set_overload_number(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subprogram_depth"]
  fn get_subprogram_depth(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subprogram_depth"]
  fn set_subprogram_depth(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subprogram_hash"]
  fn get_subprogram_hash(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subprogram_hash"]
  fn set_subprogram_hash(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_impure_depth"]
  fn get_impure_depth(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_impure_depth"]
  fn set_impure_depth(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_return_type"]
  fn get_return_type(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_return_type"]
  fn set_return_type(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_implicit_definition"]
  fn get_implicit_definition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_implicit_definition"]
  fn set_implicit_definition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_uninstantiated_subprogram_name"]
  fn get_uninstantiated_subprogram_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_uninstantiated_subprogram_name"]
  fn set_uninstantiated_subprogram_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_default_value"]
  fn get_default_value(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_default_value"]
  fn set_default_value(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_mode_view_indication"]
  fn get_mode_view_indication(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_mode_view_indication"]
  fn set_mode_view_indication(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_deferred_declaration"]
  fn get_deferred_declaration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_deferred_declaration"]
  fn set_deferred_declaration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_deferred_declaration_flag"]
  fn get_deferred_declaration_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_deferred_declaration_flag"]
  fn set_deferred_declaration_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_shared_flag"]
  fn get_shared_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_shared_flag"]
  fn set_shared_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_design_unit"]
  fn get_design_unit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_design_unit"]
  fn set_design_unit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_block_statement"]
  fn get_block_statement(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_block_statement"]
  fn set_block_statement(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_signal_driver"]
  fn get_signal_driver(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_signal_driver"]
  fn set_signal_driver(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_declaration_chain"]
  fn get_declaration_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_declaration_chain"]
  fn set_declaration_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_file_logical_name"]
  fn get_file_logical_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_file_logical_name"]
  fn set_file_logical_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_file_open_kind"]
  fn get_file_open_kind(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_file_open_kind"]
  fn set_file_open_kind(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_element_position"]
  fn get_element_position(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_element_position"]
  fn set_element_position(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_use_clause_chain"]
  fn get_use_clause_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_use_clause_chain"]
  fn set_use_clause_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_context_reference_chain"]
  fn get_context_reference_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_context_reference_chain"]
  fn set_context_reference_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_inherit_spec_chain"]
  fn get_inherit_spec_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_inherit_spec_chain"]
  fn set_inherit_spec_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_selected_name"]
  fn get_selected_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_selected_name"]
  fn set_selected_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_mode_view_name"]
  fn get_mode_view_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_mode_view_name"]
  fn set_mode_view_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_type_declarator"]
  fn get_type_declarator(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_type_declarator"]
  fn set_type_declarator(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_complete_type_definition"]
  fn get_complete_type_definition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_complete_type_definition"]
  fn set_complete_type_definition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_incomplete_type_ref_chain"]
  fn get_incomplete_type_ref_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_incomplete_type_ref_chain"]
  fn set_incomplete_type_ref_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_associated_type"]
  fn get_associated_type(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_associated_type"]
  fn set_associated_type(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_enumeration_literal_list"]
  fn get_enumeration_literal_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_enumeration_literal_list"]
  fn set_enumeration_literal_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_entity_class_entry_chain"]
  fn get_entity_class_entry_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_entity_class_entry_chain"]
  fn set_entity_class_entry_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_group_constituent_list"]
  fn get_group_constituent_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_group_constituent_list"]
  fn set_group_constituent_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_unit_chain"]
  fn get_unit_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_unit_chain"]
  fn set_unit_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_primary_unit"]
  fn get_primary_unit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_primary_unit"]
  fn set_primary_unit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_identifier"]
  fn get_identifier(n: u32) -> NameId;

  #[link_name = "vhdl__nodes__set_identifier"]
  fn set_identifier(n: u32, v: NameId);

  #[link_name = "vhdl__nodes__get_label"]
  fn get_label(n: u32) -> NameId;

  #[link_name = "vhdl__nodes__set_label"]
  fn set_label(n: u32, v: NameId);

  #[link_name = "vhdl__nodes__get_return_identifier"]
  fn get_return_identifier(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_return_identifier"]
  fn set_return_identifier(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_visible_flag"]
  fn get_visible_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_visible_flag"]
  fn set_visible_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_range_constraint"]
  fn get_range_constraint(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_range_constraint"]
  fn set_range_constraint(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_direction"]
  fn get_direction(n: u32) -> DirectionType;

  #[link_name = "vhdl__nodes__set_direction"]
  fn set_direction(n: u32, v: DirectionType);

  #[link_name = "vhdl__nodes__get_left_limit"]
  fn get_left_limit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_left_limit"]
  fn set_left_limit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_right_limit"]
  fn get_right_limit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_right_limit"]
  fn set_right_limit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_left_limit_expr"]
  fn get_left_limit_expr(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_left_limit_expr"]
  fn set_left_limit_expr(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_right_limit_expr"]
  fn get_right_limit_expr(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_right_limit_expr"]
  fn set_right_limit_expr(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_parent_type"]
  fn get_parent_type(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_parent_type"]
  fn set_parent_type(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_simple_nature"]
  fn get_simple_nature(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_simple_nature"]
  fn set_simple_nature(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_base_nature"]
  fn get_base_nature(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_base_nature"]
  fn set_base_nature(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_resolution_indication"]
  fn get_resolution_indication(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_resolution_indication"]
  fn set_resolution_indication(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_record_element_resolution_chain"]
  fn get_record_element_resolution_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_record_element_resolution_chain"]
  fn set_record_element_resolution_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_tolerance"]
  fn get_tolerance(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_tolerance"]
  fn set_tolerance(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_plus_terminal_name"]
  fn get_plus_terminal_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_plus_terminal_name"]
  fn set_plus_terminal_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_minus_terminal_name"]
  fn get_minus_terminal_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_minus_terminal_name"]
  fn set_minus_terminal_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_plus_terminal"]
  fn get_plus_terminal(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_plus_terminal"]
  fn set_plus_terminal(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_minus_terminal"]
  fn get_minus_terminal(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_minus_terminal"]
  fn set_minus_terminal(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_magnitude_expression"]
  fn get_magnitude_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_magnitude_expression"]
  fn set_magnitude_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_phase_expression"]
  fn get_phase_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_phase_expression"]
  fn set_phase_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_power_expression"]
  fn get_power_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_power_expression"]
  fn set_power_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_simultaneous_left"]
  fn get_simultaneous_left(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_simultaneous_left"]
  fn set_simultaneous_left(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_simultaneous_right"]
  fn get_simultaneous_right(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_simultaneous_right"]
  fn set_simultaneous_right(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_text_file_flag"]
  fn get_text_file_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_text_file_flag"]
  fn set_text_file_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_only_characters_flag"]
  fn get_only_characters_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_only_characters_flag"]
  fn set_only_characters_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_is_character_type"]
  fn get_is_character_type(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_is_character_type"]
  fn set_is_character_type(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_nature_staticness"]
  fn get_nature_staticness(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_nature_staticness"]
  fn set_nature_staticness(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_type_staticness"]
  fn get_type_staticness(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_type_staticness"]
  fn set_type_staticness(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_constraint_state"]
  fn get_constraint_state(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_constraint_state"]
  fn set_constraint_state(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_index_subtype_list"]
  fn get_index_subtype_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_index_subtype_list"]
  fn set_index_subtype_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_index_subtype_definition_list"]
  fn get_index_subtype_definition_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_index_subtype_definition_list"]
  fn set_index_subtype_definition_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_element_subtype_indication"]
  fn get_element_subtype_indication(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_element_subtype_indication"]
  fn set_element_subtype_indication(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_element_subtype"]
  fn get_element_subtype(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_element_subtype"]
  fn set_element_subtype(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_element_subnature_indication"]
  fn get_element_subnature_indication(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_element_subnature_indication"]
  fn set_element_subnature_indication(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_element_subnature"]
  fn get_element_subnature(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_element_subnature"]
  fn set_element_subnature(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_index_constraint_list"]
  fn get_index_constraint_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_index_constraint_list"]
  fn set_index_constraint_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_array_element_constraint"]
  fn get_array_element_constraint(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_array_element_constraint"]
  fn set_array_element_constraint(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_has_array_constraint_flag"]
  fn get_has_array_constraint_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_array_constraint_flag"]
  fn set_has_array_constraint_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_element_constraint_flag"]
  fn get_has_element_constraint_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_element_constraint_flag"]
  fn set_has_element_constraint_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_elements_declaration_list"]
  fn get_elements_declaration_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_elements_declaration_list"]
  fn set_elements_declaration_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_elements_definition_chain"]
  fn get_elements_definition_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_elements_definition_chain"]
  fn set_elements_definition_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_elements_definition_list"]
  fn get_elements_definition_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_elements_definition_list"]
  fn set_elements_definition_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_owned_elements_chain"]
  fn get_owned_elements_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_owned_elements_chain"]
  fn set_owned_elements_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_designated_type"]
  fn get_designated_type(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_designated_type"]
  fn set_designated_type(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_designated_subtype_indication"]
  fn get_designated_subtype_indication(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_designated_subtype_indication"]
  fn set_designated_subtype_indication(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_index_list"]
  fn get_index_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_index_list"]
  fn set_index_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_reference"]
  fn get_reference(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_reference"]
  fn set_reference(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_nature_declarator"]
  fn get_nature_declarator(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_nature_declarator"]
  fn set_nature_declarator(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_across_type_mark"]
  fn get_across_type_mark(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_across_type_mark"]
  fn set_across_type_mark(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_through_type_mark"]
  fn get_through_type_mark(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_through_type_mark"]
  fn set_through_type_mark(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_across_type_definition"]
  fn get_across_type_definition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_across_type_definition"]
  fn set_across_type_definition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_through_type_definition"]
  fn get_through_type_definition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_through_type_definition"]
  fn set_through_type_definition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_across_type"]
  fn get_across_type(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_across_type"]
  fn set_across_type(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_through_type"]
  fn get_through_type(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_through_type"]
  fn set_through_type(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_target"]
  fn get_target(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_target"]
  fn set_target(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_waveform_chain"]
  fn get_waveform_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_waveform_chain"]
  fn set_waveform_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_guard"]
  fn get_guard(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_guard"]
  fn set_guard(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_delay_mechanism"]
  fn get_delay_mechanism(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_delay_mechanism"]
  fn set_delay_mechanism(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_reject_time_expression"]
  fn get_reject_time_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_reject_time_expression"]
  fn set_reject_time_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_force_mode"]
  fn get_force_mode(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_force_mode"]
  fn set_force_mode(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_has_force_mode"]
  fn get_has_force_mode(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_force_mode"]
  fn set_has_force_mode(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_sensitivity_list"]
  fn get_sensitivity_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_sensitivity_list"]
  fn set_sensitivity_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_process_origin"]
  fn get_process_origin(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_process_origin"]
  fn set_process_origin(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_package_origin"]
  fn get_package_origin(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_package_origin"]
  fn set_package_origin(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_condition_clause"]
  fn get_condition_clause(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_condition_clause"]
  fn set_condition_clause(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_break_element"]
  fn get_break_element(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_break_element"]
  fn set_break_element(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_selector_quantity"]
  fn get_selector_quantity(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_selector_quantity"]
  fn set_selector_quantity(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_break_quantity"]
  fn get_break_quantity(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_break_quantity"]
  fn set_break_quantity(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_timeout_clause"]
  fn get_timeout_clause(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_timeout_clause"]
  fn set_timeout_clause(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_postponed_flag"]
  fn get_postponed_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_postponed_flag"]
  fn set_postponed_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_callees_list"]
  fn get_callees_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_callees_list"]
  fn set_callees_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_passive_flag"]
  fn get_passive_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_passive_flag"]
  fn set_passive_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_resolution_function_flag"]
  fn get_resolution_function_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_resolution_function_flag"]
  fn set_resolution_function_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_wait_state"]
  fn get_wait_state(n: u32) -> TriStateType;

  #[link_name = "vhdl__nodes__set_wait_state"]
  fn set_wait_state(n: u32, v: TriStateType);

  #[link_name = "vhdl__nodes__get_all_sensitized_state"]
  fn get_all_sensitized_state(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_all_sensitized_state"]
  fn set_all_sensitized_state(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_seen_flag"]
  fn get_seen_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_seen_flag"]
  fn set_seen_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_pure_flag"]
  fn get_pure_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_pure_flag"]
  fn set_pure_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_foreign_flag"]
  fn get_foreign_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_foreign_flag"]
  fn set_foreign_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_resolved_flag"]
  fn get_resolved_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_resolved_flag"]
  fn set_resolved_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_signal_type_flag"]
  fn get_signal_type_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_signal_type_flag"]
  fn set_signal_type_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_signal_flag"]
  fn get_has_signal_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_signal_flag"]
  fn set_has_signal_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_purity_state"]
  fn get_purity_state(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_purity_state"]
  fn set_purity_state(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_elab_flag"]
  fn get_elab_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_elab_flag"]
  fn set_elab_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_vendor_library_flag"]
  fn get_vendor_library_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_vendor_library_flag"]
  fn set_vendor_library_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_configuration_mark_flag"]
  fn get_configuration_mark_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_configuration_mark_flag"]
  fn set_configuration_mark_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_configuration_done_flag"]
  fn get_configuration_done_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_configuration_done_flag"]
  fn set_configuration_done_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_index_constraint_flag"]
  fn get_index_constraint_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_index_constraint_flag"]
  fn set_index_constraint_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_hide_implicit_flag"]
  fn get_hide_implicit_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_hide_implicit_flag"]
  fn set_hide_implicit_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_assertion_condition"]
  fn get_assertion_condition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_assertion_condition"]
  fn set_assertion_condition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_report_expression"]
  fn get_report_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_report_expression"]
  fn set_report_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_severity_expression"]
  fn get_severity_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_severity_expression"]
  fn set_severity_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_instantiated_unit"]
  fn get_instantiated_unit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_instantiated_unit"]
  fn set_instantiated_unit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_instantiated_header"]
  fn get_instantiated_header(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_instantiated_header"]
  fn set_instantiated_header(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_generic_map_aspect_chain"]
  fn get_generic_map_aspect_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_generic_map_aspect_chain"]
  fn set_generic_map_aspect_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_port_map_aspect_chain"]
  fn get_port_map_aspect_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_port_map_aspect_chain"]
  fn set_port_map_aspect_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_configuration_name"]
  fn get_configuration_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_configuration_name"]
  fn set_configuration_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_component_configuration"]
  fn get_component_configuration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_component_configuration"]
  fn set_component_configuration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_configuration_specification"]
  fn get_configuration_specification(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_configuration_specification"]
  fn set_configuration_specification(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_default_binding_indication"]
  fn get_default_binding_indication(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_default_binding_indication"]
  fn set_default_binding_indication(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_default_configuration_declaration"]
  fn get_default_configuration_declaration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_default_configuration_declaration"]
  fn set_default_configuration_declaration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_expression"]
  fn get_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_expression"]
  fn set_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_conditional_expression_chain"]
  fn get_conditional_expression_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_conditional_expression_chain"]
  fn set_conditional_expression_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_allocator_designated_type"]
  fn get_allocator_designated_type(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_allocator_designated_type"]
  fn set_allocator_designated_type(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_selected_waveform_chain"]
  fn get_selected_waveform_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_selected_waveform_chain"]
  fn set_selected_waveform_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_selected_expressions_chain"]
  fn get_selected_expressions_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_selected_expressions_chain"]
  fn set_selected_expressions_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_conditional_waveform_chain"]
  fn get_conditional_waveform_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_conditional_waveform_chain"]
  fn set_conditional_waveform_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_guard_expression"]
  fn get_guard_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_guard_expression"]
  fn set_guard_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_guard_decl"]
  fn get_guard_decl(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_guard_decl"]
  fn set_guard_decl(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_guard_sensitivity_list"]
  fn get_guard_sensitivity_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_guard_sensitivity_list"]
  fn set_guard_sensitivity_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_attribute_implicit_chain"]
  fn get_attribute_implicit_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_attribute_implicit_chain"]
  fn set_attribute_implicit_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_block_block_configuration"]
  fn get_block_block_configuration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_block_block_configuration"]
  fn set_block_block_configuration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_package_header"]
  fn get_package_header(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_package_header"]
  fn set_package_header(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_block_header"]
  fn get_block_header(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_block_header"]
  fn set_block_header(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_uninstantiated_package_name"]
  fn get_uninstantiated_package_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_uninstantiated_package_name"]
  fn set_uninstantiated_package_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_uninstantiated_package_decl"]
  fn get_uninstantiated_package_decl(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_uninstantiated_package_decl"]
  fn set_uninstantiated_package_decl(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_associated_package"]
  fn get_associated_package(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_associated_package"]
  fn set_associated_package(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_instance_source_file"]
  fn get_instance_source_file(n: u32) -> SourceFileEntry;

  #[link_name = "vhdl__nodes__set_instance_source_file"]
  fn set_instance_source_file(n: u32, v: SourceFileEntry);

  #[link_name = "vhdl__nodes__get_generate_block_configuration"]
  fn get_generate_block_configuration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_generate_block_configuration"]
  fn set_generate_block_configuration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_generate_statement_body"]
  fn get_generate_statement_body(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_generate_statement_body"]
  fn set_generate_statement_body(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_alternative_label"]
  fn get_alternative_label(n: u32) -> NameId;

  #[link_name = "vhdl__nodes__set_alternative_label"]
  fn set_alternative_label(n: u32, v: NameId);

  #[link_name = "vhdl__nodes__get_generate_else_clause"]
  fn get_generate_else_clause(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_generate_else_clause"]
  fn set_generate_else_clause(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_condition"]
  fn get_condition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_condition"]
  fn set_condition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_else_clause"]
  fn get_else_clause(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_else_clause"]
  fn set_else_clause(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_parameter_specification"]
  fn get_parameter_specification(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_parameter_specification"]
  fn set_parameter_specification(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_parent"]
  fn get_parent(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_parent"]
  fn set_parent(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_loop_label"]
  fn get_loop_label(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_loop_label"]
  fn set_loop_label(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_exit_flag"]
  fn get_exit_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_exit_flag"]
  fn set_exit_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_next_flag"]
  fn get_next_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_next_flag"]
  fn set_next_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_component_name"]
  fn get_component_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_component_name"]
  fn set_component_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_instantiation_list"]
  fn get_instantiation_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_instantiation_list"]
  fn set_instantiation_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_entity_aspect"]
  fn get_entity_aspect(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_entity_aspect"]
  fn set_entity_aspect(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_default_entity_aspect"]
  fn get_default_entity_aspect(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_default_entity_aspect"]
  fn set_default_entity_aspect(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_binding_indication"]
  fn get_binding_indication(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_binding_indication"]
  fn set_binding_indication(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_named_entity"]
  fn get_named_entity(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_named_entity"]
  fn set_named_entity(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_referenced_name"]
  fn get_referenced_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_referenced_name"]
  fn set_referenced_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_expr_staticness"]
  fn get_expr_staticness(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_expr_staticness"]
  fn set_expr_staticness(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_scalar_size"]
  fn get_scalar_size(n: u32) -> ScalarSize;

  #[link_name = "vhdl__nodes__set_scalar_size"]
  fn set_scalar_size(n: u32, v: ScalarSize);

  #[link_name = "vhdl__nodes__get_error_origin"]
  fn get_error_origin(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_error_origin"]
  fn set_error_origin(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_operand"]
  fn get_operand(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_operand"]
  fn set_operand(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_left"]
  fn get_left(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_left"]
  fn set_left(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_right"]
  fn get_right(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_right"]
  fn set_right(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_unit_name"]
  fn get_unit_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_unit_name"]
  fn set_unit_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_name"]
  fn get_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_name"]
  fn set_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_group_template_name"]
  fn get_group_template_name(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_group_template_name"]
  fn set_group_template_name(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_name_staticness"]
  fn get_name_staticness(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_name_staticness"]
  fn set_name_staticness(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_prefix"]
  fn get_prefix(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_prefix"]
  fn set_prefix(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_signature_prefix"]
  fn get_signature_prefix(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_signature_prefix"]
  fn set_signature_prefix(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_external_pathname"]
  fn get_external_pathname(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_external_pathname"]
  fn set_external_pathname(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_pathname_suffix"]
  fn get_pathname_suffix(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_pathname_suffix"]
  fn set_pathname_suffix(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_pathname_expression"]
  fn get_pathname_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_pathname_expression"]
  fn set_pathname_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_in_formal_flag"]
  fn get_in_formal_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_in_formal_flag"]
  fn set_in_formal_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_inertial_flag"]
  fn get_inertial_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_inertial_flag"]
  fn set_inertial_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_slice_subtype"]
  fn get_slice_subtype(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_slice_subtype"]
  fn set_slice_subtype(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_suffix"]
  fn get_suffix(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_suffix"]
  fn set_suffix(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_index_subtype"]
  fn get_index_subtype(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_index_subtype"]
  fn set_index_subtype(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_parameter"]
  fn get_parameter(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_parameter"]
  fn set_parameter(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_parameter_2"]
  fn get_parameter_2(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_parameter_2"]
  fn set_parameter_2(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_parameter_3"]
  fn get_parameter_3(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_parameter_3"]
  fn set_parameter_3(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_parameter_4"]
  fn get_parameter_4(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_parameter_4"]
  fn set_parameter_4(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_attr_chain"]
  fn get_attr_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_attr_chain"]
  fn set_attr_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_actual_type"]
  fn get_actual_type(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_actual_type"]
  fn set_actual_type(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_actual_type_definition"]
  fn get_actual_type_definition(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_actual_type_definition"]
  fn set_actual_type_definition(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_association_chain"]
  fn get_association_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_association_chain"]
  fn set_association_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_individual_association_chain"]
  fn get_individual_association_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_individual_association_chain"]
  fn set_individual_association_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subprogram_association_chain"]
  fn get_subprogram_association_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subprogram_association_chain"]
  fn set_subprogram_association_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_aggregate_info"]
  fn get_aggregate_info(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_aggregate_info"]
  fn set_aggregate_info(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_sub_aggregate_info"]
  fn get_sub_aggregate_info(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_sub_aggregate_info"]
  fn set_sub_aggregate_info(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_aggr_dynamic_flag"]
  fn get_aggr_dynamic_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_aggr_dynamic_flag"]
  fn set_aggr_dynamic_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_aggr_min_length"]
  fn get_aggr_min_length(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_aggr_min_length"]
  fn set_aggr_min_length(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_aggr_low_limit"]
  fn get_aggr_low_limit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_aggr_low_limit"]
  fn set_aggr_low_limit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_aggr_high_limit"]
  fn get_aggr_high_limit(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_aggr_high_limit"]
  fn set_aggr_high_limit(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_aggr_others_flag"]
  fn get_aggr_others_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_aggr_others_flag"]
  fn set_aggr_others_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_aggr_named_flag"]
  fn get_aggr_named_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_aggr_named_flag"]
  fn set_aggr_named_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_aggregate_expand_flag"]
  fn get_aggregate_expand_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_aggregate_expand_flag"]
  fn set_aggregate_expand_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_determined_aggregate_flag"]
  fn get_determined_aggregate_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_determined_aggregate_flag"]
  fn set_determined_aggregate_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_association_choices_chain"]
  fn get_association_choices_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_association_choices_chain"]
  fn set_association_choices_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_case_statement_alternative_chain"]
  fn get_case_statement_alternative_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_case_statement_alternative_chain"]
  fn set_case_statement_alternative_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_matching_flag"]
  fn get_matching_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_matching_flag"]
  fn set_matching_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_choice_staticness"]
  fn get_choice_staticness(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_choice_staticness"]
  fn set_choice_staticness(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_procedure_call"]
  fn get_procedure_call(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_procedure_call"]
  fn set_procedure_call(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_implementation"]
  fn get_implementation(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_implementation"]
  fn set_implementation(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_parameter_association_chain"]
  fn get_parameter_association_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_parameter_association_chain"]
  fn set_parameter_association_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_method_object"]
  fn get_method_object(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_method_object"]
  fn set_method_object(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subtype_type_mark"]
  fn get_subtype_type_mark(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subtype_type_mark"]
  fn set_subtype_type_mark(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_subnature_nature_mark"]
  fn get_subnature_nature_mark(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_subnature_nature_mark"]
  fn set_subnature_nature_mark(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_type_conversion_subtype"]
  fn get_type_conversion_subtype(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_type_conversion_subtype"]
  fn set_type_conversion_subtype(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_type_mark"]
  fn get_type_mark(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_type_mark"]
  fn set_type_mark(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_file_type_mark"]
  fn get_file_type_mark(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_file_type_mark"]
  fn set_file_type_mark(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_return_type_mark"]
  fn get_return_type_mark(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_return_type_mark"]
  fn set_return_type_mark(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_has_disconnect_flag"]
  fn get_has_disconnect_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_disconnect_flag"]
  fn set_has_disconnect_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_active_flag"]
  fn get_has_active_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_active_flag"]
  fn set_has_active_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_is_within_flag"]
  fn get_is_within_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_is_within_flag"]
  fn set_is_within_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_type_marks_list"]
  fn get_type_marks_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_type_marks_list"]
  fn set_type_marks_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_implicit_alias_flag"]
  fn get_implicit_alias_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_implicit_alias_flag"]
  fn set_implicit_alias_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_alias_signature"]
  fn get_alias_signature(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_alias_signature"]
  fn set_alias_signature(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_attribute_signature"]
  fn get_attribute_signature(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_attribute_signature"]
  fn set_attribute_signature(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_overload_list"]
  fn get_overload_list(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_overload_list"]
  fn set_overload_list(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_simple_name_identifier"]
  fn get_simple_name_identifier(n: u32) -> NameId;

  #[link_name = "vhdl__nodes__set_simple_name_identifier"]
  fn set_simple_name_identifier(n: u32, v: NameId);

  #[link_name = "vhdl__nodes__get_simple_name_subtype"]
  fn get_simple_name_subtype(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_simple_name_subtype"]
  fn set_simple_name_subtype(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_protected_type_body"]
  fn get_protected_type_body(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_protected_type_body"]
  fn set_protected_type_body(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_protected_type_declaration"]
  fn get_protected_type_declaration(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_protected_type_declaration"]
  fn set_protected_type_declaration(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_use_flag"]
  fn get_use_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_use_flag"]
  fn set_use_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_elaborated_flag"]
  fn get_elaborated_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_elaborated_flag"]
  fn set_elaborated_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_end_has_reserved_id"]
  fn get_end_has_reserved_id(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_end_has_reserved_id"]
  fn set_end_has_reserved_id(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_end_has_identifier"]
  fn get_end_has_identifier(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_end_has_identifier"]
  fn set_end_has_identifier(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_end_has_postponed"]
  fn get_end_has_postponed(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_end_has_postponed"]
  fn set_end_has_postponed(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_begin"]
  fn get_has_begin(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_begin"]
  fn set_has_begin(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_end"]
  fn get_has_end(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_end"]
  fn set_has_end(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_is"]
  fn get_has_is(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_is"]
  fn set_has_is(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_pure"]
  fn get_has_pure(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_pure"]
  fn set_has_pure(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_body"]
  fn get_has_body(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_body"]
  fn set_has_body(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_parameter"]
  fn get_has_parameter(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_parameter"]
  fn set_has_parameter(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_component"]
  fn get_has_component(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_component"]
  fn set_has_component(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_identifier_list"]
  fn get_has_identifier_list(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_identifier_list"]
  fn set_has_identifier_list(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_mode"]
  fn get_has_mode(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_mode"]
  fn set_has_mode(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_class"]
  fn get_has_class(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_class"]
  fn set_has_class(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_has_delay_mechanism"]
  fn get_has_delay_mechanism(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_has_delay_mechanism"]
  fn set_has_delay_mechanism(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_suspend_flag"]
  fn get_suspend_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_suspend_flag"]
  fn set_suspend_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_covered_flag"]
  fn get_covered_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_covered_flag"]
  fn set_covered_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_stop_flag"]
  fn get_stop_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_stop_flag"]
  fn set_stop_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_is_ref"]
  fn get_is_ref(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_is_ref"]
  fn set_is_ref(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_is_forward_ref"]
  fn get_is_forward_ref(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_is_forward_ref"]
  fn set_is_forward_ref(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_psl_property"]
  fn get_psl_property(n: u32) -> PSLNode;

  #[link_name = "vhdl__nodes__set_psl_property"]
  fn set_psl_property(n: u32, v: PSLNode);

  #[link_name = "vhdl__nodes__get_psl_sequence"]
  fn get_psl_sequence(n: u32) -> PSLNode;

  #[link_name = "vhdl__nodes__set_psl_sequence"]
  fn set_psl_sequence(n: u32, v: PSLNode);

  #[link_name = "vhdl__nodes__get_psl_declaration"]
  fn get_psl_declaration(n: u32) -> PSLNode;

  #[link_name = "vhdl__nodes__set_psl_declaration"]
  fn set_psl_declaration(n: u32, v: PSLNode);

  #[link_name = "vhdl__nodes__get_psl_expression"]
  fn get_psl_expression(n: u32) -> PSLNode;

  #[link_name = "vhdl__nodes__set_psl_expression"]
  fn set_psl_expression(n: u32, v: PSLNode);

  #[link_name = "vhdl__nodes__get_psl_boolean"]
  fn get_psl_boolean(n: u32) -> PSLNode;

  #[link_name = "vhdl__nodes__set_psl_boolean"]
  fn set_psl_boolean(n: u32, v: PSLNode);

  #[link_name = "vhdl__nodes__get_psl_clock"]
  fn get_psl_clock(n: u32) -> PSLNode;

  #[link_name = "vhdl__nodes__set_psl_clock"]
  fn set_psl_clock(n: u32, v: PSLNode);

  #[link_name = "vhdl__nodes__get_psl_abort"]
  fn get_psl_abort(n: u32) -> PSLNode;

  #[link_name = "vhdl__nodes__set_psl_abort"]
  fn set_psl_abort(n: u32, v: PSLNode);

  #[link_name = "vhdl__nodes__get_psl_nfa"]
  fn get_psl_nfa(n: u32) -> PSLNFA;

  #[link_name = "vhdl__nodes__set_psl_nfa"]
  fn set_psl_nfa(n: u32, v: PSLNFA);

  #[link_name = "vhdl__nodes__get_psl_nbr_states"]
  fn get_psl_nbr_states(n: u32) -> i32;

  #[link_name = "vhdl__nodes__set_psl_nbr_states"]
  fn set_psl_nbr_states(n: u32, v: i32);

  #[link_name = "vhdl__nodes__get_psl_clock_sensitivity"]
  fn get_psl_clock_sensitivity(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_psl_clock_sensitivity"]
  fn set_psl_clock_sensitivity(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_psl_eos_flag"]
  fn get_psl_eos_flag(n: u32) -> bool;

  #[link_name = "vhdl__nodes__set_psl_eos_flag"]
  fn set_psl_eos_flag(n: u32, v: bool);

  #[link_name = "vhdl__nodes__get_count_expression"]
  fn get_count_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_count_expression"]
  fn set_count_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_clock_expression"]
  fn get_clock_expression(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_clock_expression"]
  fn set_clock_expression(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_default_clock"]
  fn get_default_clock(n: u32) -> PSLNode;

  #[link_name = "vhdl__nodes__set_default_clock"]
  fn set_default_clock(n: u32, v: PSLNode);

  #[link_name = "vhdl__nodes__get_foreign_node"]
  fn get_foreign_node(n: u32) -> i32;

  #[link_name = "vhdl__nodes__set_foreign_node"]
  fn set_foreign_node(n: u32, v: i32);

  #[link_name = "vhdl__nodes__get_suspend_state_index"]
  fn get_suspend_state_index(n: u32) -> i32;

  #[link_name = "vhdl__nodes__set_suspend_state_index"]
  fn set_suspend_state_index(n: u32, v: i32);

  #[link_name = "vhdl__nodes__get_suspend_state_chain"]
  fn get_suspend_state_chain(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_suspend_state_chain"]
  fn set_suspend_state_chain(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_suspend_state_last"]
  fn get_suspend_state_last(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_suspend_state_last"]
  fn set_suspend_state_last(n: u32, v: Iir);

  #[link_name = "vhdl__nodes__get_suspend_state_decl"]
  fn get_suspend_state_decl(n: u32) -> Iir;

  #[link_name = "vhdl__nodes__set_suspend_state_decl"]
  fn set_suspend_state_decl(n: u32, v: Iir);

}
