(function() {var implementors = {};
implementors["tf_serving"] = ["impl MessageStatic for <a class=\"struct\" href=\"tf_serving/classification/struct.Class.html\" title=\"struct tf_serving::classification::Class\">Class</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/classification/struct.Classifications.html\" title=\"struct tf_serving::classification::Classifications\">Classifications</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/classification/struct.ClassificationResult.html\" title=\"struct tf_serving::classification::ClassificationResult\">ClassificationResult</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/classification/struct.ClassificationRequest.html\" title=\"struct tf_serving::classification::ClassificationRequest\">ClassificationRequest</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/classification/struct.ClassificationResponse.html\" title=\"struct tf_serving::classification::ClassificationResponse\">ClassificationResponse</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/feature/struct.BytesList.html\" title=\"struct tf_serving::feature::BytesList\">BytesList</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/feature/struct.FloatList.html\" title=\"struct tf_serving::feature::FloatList\">FloatList</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/feature/struct.Int64List.html\" title=\"struct tf_serving::feature::Int64List\">Int64List</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/feature/struct.Feature.html\" title=\"struct tf_serving::feature::Feature\">Feature</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/feature/struct.Features.html\" title=\"struct tf_serving::feature::Features\">Features</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/feature/struct.FeatureList.html\" title=\"struct tf_serving::feature::FeatureList\">FeatureList</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/feature/struct.FeatureLists.html\" title=\"struct tf_serving::feature::FeatureLists\">FeatureLists</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/function/struct.FunctionDefLibrary.html\" title=\"struct tf_serving::function::FunctionDefLibrary\">FunctionDefLibrary</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/function/struct.FunctionDef.html\" title=\"struct tf_serving::function::FunctionDef\">FunctionDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/function/struct.GradientDef.html\" title=\"struct tf_serving::function::GradientDef\">GradientDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/graph/struct.GraphDef.html\" title=\"struct tf_serving::graph::GraphDef\">GraphDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/inference/struct.InferenceTask.html\" title=\"struct tf_serving::inference::InferenceTask\">InferenceTask</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/inference/struct.InferenceResult.html\" title=\"struct tf_serving::inference::InferenceResult\">InferenceResult</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/inference/struct.MultiInferenceRequest.html\" title=\"struct tf_serving::inference::MultiInferenceRequest\">MultiInferenceRequest</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/inference/struct.MultiInferenceResponse.html\" title=\"struct tf_serving::inference::MultiInferenceResponse\">MultiInferenceResponse</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.MetaGraphDef.html\" title=\"struct tf_serving::meta_graph::MetaGraphDef\">MetaGraphDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.MetaGraphDef_MetaInfoDef.html\" title=\"struct tf_serving::meta_graph::MetaGraphDef_MetaInfoDef\">MetaGraphDef_MetaInfoDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.CollectionDef.html\" title=\"struct tf_serving::meta_graph::CollectionDef\">CollectionDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.CollectionDef_NodeList.html\" title=\"struct tf_serving::meta_graph::CollectionDef_NodeList\">CollectionDef_NodeList</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.CollectionDef_BytesList.html\" title=\"struct tf_serving::meta_graph::CollectionDef_BytesList\">CollectionDef_BytesList</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.CollectionDef_Int64List.html\" title=\"struct tf_serving::meta_graph::CollectionDef_Int64List\">CollectionDef_Int64List</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.CollectionDef_FloatList.html\" title=\"struct tf_serving::meta_graph::CollectionDef_FloatList\">CollectionDef_FloatList</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.CollectionDef_AnyList.html\" title=\"struct tf_serving::meta_graph::CollectionDef_AnyList\">CollectionDef_AnyList</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.TensorInfo.html\" title=\"struct tf_serving::meta_graph::TensorInfo\">TensorInfo</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.TensorInfo_CooSparse.html\" title=\"struct tf_serving::meta_graph::TensorInfo_CooSparse\">TensorInfo_CooSparse</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.SignatureDef.html\" title=\"struct tf_serving::meta_graph::SignatureDef\">SignatureDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/meta_graph/struct.AssetFileDef.html\" title=\"struct tf_serving::meta_graph::AssetFileDef\">AssetFileDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/node_def/struct.NodeDef.html\" title=\"struct tf_serving::node_def::NodeDef\">NodeDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/op_def/struct.OpDef.html\" title=\"struct tf_serving::op_def::OpDef\">OpDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/op_def/struct.OpDef_ArgDef.html\" title=\"struct tf_serving::op_def::OpDef_ArgDef\">OpDef_ArgDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/op_def/struct.OpDef_AttrDef.html\" title=\"struct tf_serving::op_def::OpDef_AttrDef\">OpDef_AttrDef</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/op_def/struct.OpDeprecation.html\" title=\"struct tf_serving::op_def::OpDeprecation\">OpDeprecation</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/op_def/struct.OpList.html\" title=\"struct tf_serving::op_def::OpList\">OpList</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/predict/struct.PredictRequest.html\" title=\"struct tf_serving::predict::PredictRequest\">PredictRequest</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/predict/struct.PredictResponse.html\" title=\"struct tf_serving::predict::PredictResponse\">PredictResponse</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/regression/struct.Regression.html\" title=\"struct tf_serving::regression::Regression\">Regression</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/regression/struct.RegressionResult.html\" title=\"struct tf_serving::regression::RegressionResult\">RegressionResult</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/regression/struct.RegressionRequest.html\" title=\"struct tf_serving::regression::RegressionRequest\">RegressionRequest</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/regression/struct.RegressionResponse.html\" title=\"struct tf_serving::regression::RegressionResponse\">RegressionResponse</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/resource_handle/struct.ResourceHandleProto.html\" title=\"struct tf_serving::resource_handle::ResourceHandleProto\">ResourceHandleProto</a>","impl MessageStatic for <a class=\"struct\" href=\"tf_serving/versions/struct.VersionDef.html\" title=\"struct tf_serving::versions::VersionDef\">VersionDef</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()