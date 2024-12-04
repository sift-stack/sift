from pathlib import Path
from typing import Dict, Optional, cast

import click
from dotenv import dotenv_values
from sift.ingestion_configs.v1.ingestion_configs_pb2 import IngestionConfig
from sift.ping.v1.ping_pb2 import PingRequest
from sift.ping.v1.ping_pb2_grpc import PingServiceStub
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion._internal.ingestion_config import (
    get_ingestion_config_by_client_key,
    get_ingestion_config_flows,
    list_ingestion_configs,
)

DEFUALT_AUTH_PATH = Path.home() / ".sift" / ".auth.env"


@click.group()
@click.option(
    "--auth_file",
    default=DEFUALT_AUTH_PATH,
    show_default=True,
    type=click.Path(),
    help="Path to the auth file.",
)
@click.pass_context
def sift_cli(ctx: click.Context, auth_file: str):
    credentials: Dict[str, Optional[str]] = {}

    if auth_file:
        credentials = dotenv_values(auth_file)
    else:
        if DEFUALT_AUTH_PATH.exists():
            credentials = dotenv_values(DEFUALT_AUTH_PATH)
        else:
            raise click.ClickException(f"Auth file not found at {DEFUALT_AUTH_PATH}")

    if not credentials.get("SIFT_API_KEY"):
        raise click.ClickException("Auth file must contain SIFT_API_KEY")

    if not credentials.get("BASE_GRPC_URI"):
        raise click.ClickException("Auth file must contain BASE_GRPC_URI")

    validated_credentials = cast(Dict[str, str], credentials)

    channel_config: SiftChannelConfig = {
        "apikey": validated_credentials["SIFT_API_KEY"],
        "uri": validated_credentials["BASE_GRPC_URI"],
    }

    ctx.obj["channel"] = ctx.with_resource(use_sift_channel(channel_config))


################
# Ping Service #
################


@sift_cli.command()
@click.pass_context
def ping(ctx):
    """Test the connection to the Sift API."""
    click.echo(PingServiceStub(ctx.obj["channel"]).Ping(PingRequest()))


############################
# Ingestion Config Service #
############################


@sift_cli.command(name="list-ingestion-configs")
@click.option(
    "--filter",
    default=None,
    help="A Common Expression Language (CEL) filter string.",
)
@click.pass_context
def list_ingestion_configs_command(ctx: click.Context, filter: str):
    """List ingestion configs using an optional filter."""
    configs = list_ingestion_configs(ctx.obj["channel"], filter)
    if configs:
        click.echo("Results:\n")
        for config in configs:
            click.echo(config)
    else:
        click.echo("No results found.")


@sift_cli.command(name="get-ingestion-config")
@click.argument(
    "client_key",
)
@click.option(
    "--list_flows",
    is_flag=True,
    help="List flow information for the ingestion config.",
)
@click.pass_context
def get_ingestion_config_by_client_key_command(
    ctx: click.Context, client_key: str, list_flows: bool
):
    """Return an ingestion config by its client key."""
    result: Optional[IngestionConfig] = get_ingestion_config_by_client_key(
        ctx.obj["channel"], client_key
    )
    if result:
        click.echo(result)
        if list_flows:
            for flow in get_ingestion_config_flows(ctx.obj["channel"], result.ingestion_config_id):
                click.echo(flow)

    else:
        click.echo("No results found.")


def main():
    sift_cli(obj={})


if __name__ == "__main__":
    main()
