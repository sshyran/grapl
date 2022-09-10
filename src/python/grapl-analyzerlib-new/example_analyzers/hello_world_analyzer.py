from grapl_analyzerlib_new.analyzer import Analyzer, AnalyzerServiceConfig, serve_analyzer, AnalyzerContext
from python_proto.api.graph_query.v1beta1.messages import StringFilter, StringOperation
from python_proto.api.plugin_sdk.analyzers.v1beta1.messages import AnalyzerName, ExecutionHit
from grapl_analyzerlib_new.query_and_views import NodeQuery, NodeView
from python_proto.grapl.common.v1beta1.messages import PropertyName

class HelloWorldAnalyzer(Analyzer):
    @staticmethod
    def query() -> NodeQuery:
        NodeQuery.with_string_filters(
            property_name=PropertyName(value="process_name"),
            filters=[
                StringFilter(
                    operation=StringOperation.EQUAL,
                    value="chrome.exe",
                    negated=False
                ),
            ]
        )

    async def analyze(
        self, matched: NodeView, ctx: AnalyzerContext
    ) -> ExecutionHit | None:
        pass

    async def add_context(self, matched: NodeView, ctx: AnalyzerContext) -> None:
        pass


async def main() -> None:
    analyzer = HelloWorldAnalyzer()
    # Perhaps `serve_analyzer` should just take `(analyzer=analyzer)`?
    serve_analyzer(
        analyzer_name=AnalyzerName(value="hello_world"), # Why is this configured here?
        analyzer=analyzer,
        service_config=AnalyzerServiceConfig.from_env(),
    )
    
if __name__ == "__main__":
    main()