import React from "react";

import Popover from "@mui/material/Popover";
import Card from "@mui/material/Card";
import CardActionArea from "@mui/material/CardActionArea";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";

import { notificationsStyles } from "./styles";

const useStyles: any = notificationsStyles;

export default function LoginNotification() {
    const classes = useStyles();

    return (
        <Popover
            open={true}
            anchorOrigin={{
                vertical: "top",
                horizontal: "right",
            }}
            transformOrigin={{
                vertical: "top",
                horizontal: "right",
            }}
        >
            <Card variant="outlined" className={classes.root}>
                <CardActionArea>
                    <CardContent>
                        <Typography
                            variant="body2"
                            color="textSecondary"
                            component="p"
                        >
                            You are not logged in and changes cannot be saved.
                        </Typography>
                    </CardContent>
                </CardActionArea>

                <CardActions>
                    <Button
                        className={classes.button}
                        size="small"
                        onClick={() => {
                            window.history.replaceState("#/", "", "#/login");
                            window.location.reload();
                        }}
                    >
                        {" "}
                        Login{" "}
                    </Button>
                </CardActions>
            </Card>
        </Popover>
    );
}
