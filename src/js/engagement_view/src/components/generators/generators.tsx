import React from "react";
import { useTheme } from "@mui/material/styles";

import Box from "@mui/material/Box";
import Paper from "@mui/material/Paper";
import { DataGrid, GridColDef } from "@mui/x-data-grid";

import { Field, Form, Formik } from "formik";

//Paper (Notifications) Icons
import ViewListIcon from "@mui/icons-material/ViewList";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import ErrorIcon from "@mui/icons-material/Error";
import NotificationsIcon from "@mui/icons-material/Notifications";

import { useStyles } from "../styles/analyzersAndGeneratorsStyles";
import "../../index.css";
import { NavigationDrawer } from "../reusableComponents/drawer";

const MetricContainer = () => {
    const classes = useStyles();

    return (
        <Box
            sx={{
                display: "flex",
                flexWrap: "wrap",
                "& > :not(style)": {
                    m: 1,
                    margin: ".75rem",
                },
            }}
        >
            <Paper
                className={classes.metricContainer}
                sx={{ backgroundColor: "#212936", color: "#56657F" }}
                elevation={3}
            >
                <div className={classes.metricsText}>
                    <ViewListIcon
                        style={{ fontSize: "30px" }}
                        className={classes.metricsIcon}
                    />
                    <h1 className={classes.metricCount}>0</h1>
                    <p>List All </p>
                </div>
            </Paper>

            <Paper
                className={classes.metricContainer}
                sx={{ backgroundColor: "#212936", color: "#56657F" }}
                elevation={3}
            >
                <div className={classes.metricsText}>
                    <CheckCircleIcon
                        style={{ fontSize: "30px" }}
                        className={classes.metricsIcon}
                    />
                    <h1 className={classes.metricCount}>0</h1>
                    <p>Healthy</p>
                </div>
            </Paper>

            <Paper
                className={classes.metricContainer}
                sx={{ backgroundColor: "#212936", color: "#56657F" }}
                elevation={3}
            >
                <div className={classes.metricsText}>
                    <ErrorIcon
                        style={{ fontSize: "30px" }}
                        className={classes.metricsIcon}
                    />
                    <h1 className={classes.metricCount}>0</h1>
                    <p>Crashing</p>
                </div>
            </Paper>

            <Paper
                className={classes.metricContainer}
                sx={{ backgroundColor: "#212936", color: "#56657F" }}
                elevation={3}
            >
                <div className={classes.metricsText}>
                    <NotificationsIcon
                        style={{ fontSize: "30px" }}
                        className={classes.metricsIcon}
                    />
                    <h1 className={classes.metricCount}>0</h1>
                    <p>Alerts</p>
                </div>
            </Paper>
        </Box>
    );
};

export const UploadForm = () => {
    const classes = useStyles();
    return (
        <div className={classes.uploadFormContainer}>
            <h3 className={classes.header}> Upload Generator</h3>
            <Formik
                initialValues={{ name: "" }} // empty files
                onSubmit={(values, actions) => {
                    setTimeout(() => {
                        // placeholder for now
                        alert(JSON.stringify(values, null, 2));
                        actions.setSubmitting(false);
                    }, 1000);
                }}
            >
                {(props) => (
                    <Form
                        onSubmit={props.handleSubmit}
                        className={classes.uploadForm}
                    >
                        <Field
                            name="plugin"
                            directory=""
                            webkitdirectory=""
                            mozdirectory=""
                            type="file"
                            multiple
                            placeholder="Plugin"
                            onChange={props.handleChange} // upload form doesn't work (obv)
                        />
                        <button type="submit" className={classes.submitBtn}>
                            UPLOAD
                        </button>
                    </Form>
                )}
            </Formik>
        </div>
    );
};

export const GeneratorsListTable = () => {
    const classes = useStyles();

    const columns: GridColDef[] = [
        {
            field: "generatorName",
            headerName: "Generator Name",
            width: 200,
            editable: false,
        },
        {
            field: "date",
            headerName: "Date",
            width: 150,
            editable: false,
        },
    ];

    const rows = [
        { id: 1, generatorName: "Sysmon", date: "03/17/22" },
        { id: 2, generatorName: "OS Query", date: "03/17/22" },
        { id: 3, generatorName: "Cloudtrail", date: "03/17/22" },
        { id: 4, generatorName: "GuardDuty", date: "03/17/22" },
        { id: 5, generatorName: "GuardDuty", date: "03/17/22" },
    ];

    return (
        <div className={classes.generatorsListTable}>
            <DataGrid
                sx={{
                    bgcolor: "#212936",
                    color: "#FFF",
                    boxShadow: 1,
                    border: 0,
                    borderRadius: 2,
                    p: 2,
                    minWidth: 300,
                    "& 	.MuiDataGrid-columnHeader": {
                        color: "#8997B1",
                    },
                    "& .MuiDataGrid-columnSeparator": {
                        visibility: "hidden",
                    },
                    "& .MuiDataGrid-sortIcon": {
                        color: "#8997B1",
                    },
                    "& .MuiCheckbox-root": {
                        color: "#8997B1",
                    },
                    "& .MuiIconButton-root": {
                        color: "#8997B1",
                    },
                    "& .MuiTablePagination-displayedRows": {
                        color: "#8997B1",
                    },
                    "& .MuiTablePagination-actions": {
                        color: "#8997B1",
                    },
                }}
                rows={rows}
                columns={columns}
                pageSize={5}
                rowsPerPageOptions={[5]}
                checkboxSelection
                disableSelectionOnClick
            />
        </div>
    );
};

const Generators = () => {
    const theme = useTheme();
    const classes = useStyles();

    return (
        <Box className={classes.root} sx={{ display: "flex" }}>
            <NavigationDrawer />

            <div className={classes.metricsAndUploadContainer}>
                <MetricContainer></MetricContainer>
                <UploadForm></UploadForm>
            </div>
            <div>
                <GeneratorsListTable></GeneratorsListTable>
            </div>
        </Box>
    );
};

export default Generators;
