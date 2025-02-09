from functools import lru_cache
from typing import Any, cast
from unittest import TestCase

import hypothesis
import pytest
from grapl_analyzerlib.test_utils.strategies.asset_view_strategy import (
    AssetProps,
    asset_props_strategy,
)
from grapl_common.debugger.vsc_debugger import wait_for_vsc_debugger
from grapl_common.grapl_logger import get_module_grapl_logger
from grapl_tests_common.clients.graphql_endpoint_client import GraphqlEndpointClient
from grapl_tests_common.clients.grapl_web_client import GraplWebClient
from grapl_tests_common.scenarios.create_lens_with_nodes_in_scope import *

LOGGER = get_module_grapl_logger()

GqlLensDict = dict[str, Any]


@lru_cache(maxsize=1)
def hacky_memoized_actix_session() -> str:
    """
    Doesn't work as a Hypothesis given because we try to evaluate those, even
    for unit tests - despite not having the right env vars for it at unit test
    time.

    We memoize it because `get_actix_session` is CPU-bounded, lots of Argon
    number crunching.
    That doesn't mix well with Hypothesis's whole "let's run 100 tests" shtick.
    """
    return GraplWebClient().get_actix_session()


@pytest.mark.integration_test
class TestGraphqlEndpoint(TestCase):
    def __init__(self, *args, **kwargs) -> None:  # type: ignore
        super().__init__(*args, **kwargs)
        wait_for_vsc_debugger(service="graphql_endpoint_tests")

    @hypothesis.given(
        asset_props=asset_props_strategy(),
    )
    @hypothesis.settings(deadline=None, max_examples=25)
    def test_create_lens_shows_up_in_graphql(
        self,
        asset_props: AssetProps,
    ) -> None:
        graph_client = GraphClient()
        actix_session = hacky_memoized_actix_session()
        graphql_client = GraphqlEndpointClient(actix_session=actix_session)

        lens = create_lens_with_nodes_in_scope(self, graph_client, asset_props)
        lens_name = lens.get_lens_name()
        assert lens_name

        # Check that this lens shows up in the "show all lenses" view
        # NOTE: This test could be somewhat finicky, since it just gets the first 1000 lenses
        gql_lenses = _query_graphql_endpoint_for_lenses(graphql_client)
        assert lens_name in [l["lens_name"] for l in gql_lenses]

        # Query by that lens name
        gql_lens = graphql_client.query_for_scope(lens_name)
        LOGGER.info(gql_lens)
        # For some reason, upon create, `lens.uid` comes back as a string like "0x5"
        assert gql_lens["uid"] == int(lens.uid, 0)  # type: ignore
        # Check the nodes in scope
        assert len(gql_lens["scope"]) == 1
        # Ensure we strip the Entity and Base types
        asset_node = gql_lens["scope"][0]
        assert asset_node["dgraph_type"] == ["Asset"]
        assert asset_node["hostname"] == asset_props["hostname"]
        # Ensure we send along the Display
        assert asset_node["display"] == asset_props["hostname"]

    def test_describe_asset_type(
        self,
    ) -> None:
        actix_session = hacky_memoized_actix_session()
        graphql_client = GraphqlEndpointClient(actix_session=actix_session)

        result = graphql_client.query_type("Asset")
        assert result["name"] == "Asset"

        uid_looks_like = {"name": "uid", "type": {"name": "Int", "kind": "SCALAR"}}
        assert any(x for x in result["fields"] if x == uid_looks_like)


def _query_graphql_endpoint_for_lenses(
    gql_client: GraphqlEndpointClient,
) -> list[GqlLensDict]:
    # Just get *all* lenses
    query = """
    {
        lenses(first: 1000, offset: 0) {
            uid,
            node_key,
            lens_name,
            score,
            lens_type,
        }
    }
    """
    resp = gql_client.query(query)
    return cast(list[dict[str, Any]], resp["lenses"])
