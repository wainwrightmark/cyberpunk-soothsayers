use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};
use yew_router::prelude::use_navigator;

use crate::web::app::Route;

#[derive(Properties, Debug, PartialEq, Clone, Copy)]
pub struct LogoProps {
    pub clickable: bool,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    let navigator = use_navigator().unwrap();

    let style = if props.clickable {
        "margin:auto; display: block; cursor:pointer;"
    } else {
        "margin:auto; display: block; pointer-events: none;"
    };

    let onclick = {
        let props = *props;
        Callback::from(move |_: MouseEvent| {
            if props.clickable {
                navigator.push(&Route::Opening {});
            }
        })
    };

    html!(
            <svg xmlns="http://www.w3.org/2000/svg" width="200" {style}  viewBox="0 0 2763 1833" {onclick}>
        <path id="full_logo_small_1" data-name="full logo small 1" class="cls-1" d="M2205.83,1976.41c-10.84-.2-22.13-3.64-31.6-6.11-17.81-4.66-36.22-5.83-53.02-10.2-37.29-9.7-75.93-11.63-112.15-21.41l-21.41-3.06c-18.88-5.04-39.22-5.94-59.14-11.21-4.42-1.17-14.51-4.75-18.35-3.06-7.74,5.74-11.92,17.36-17.33,25.49-11.91,17.88-24.19,36.07-36.71,54.03-25.49,36.57-49.64,74.3-75.44,111.12-13.12,18.72-29.17,37.37-39.77,58.11l-1.02-1.02q-84.615-124.365-169.24-248.75c-20.79,6.7-43.18,6.51-65.25,12.24-49.74,12.89-104.17,17.81-153.96,30.58-25.06,6.43-52.16,5.48-75.44,14.27v-6.11c3.59-5.74,2.13-13.92,4.07-21.41,4.88-18.81,6.05-39.14,11.22-59.13,17.5-67.7,26.39-140.9,40.78-210.01-16.49-9.32-31.65-21.9-46.9-32.63-9.66-6.79-19.89-11.56-29.57-18.35-24.9-17.45-50.2-36.07-75.44-53.01-21.34-14.32-42.19-28.8-63.22-42.82-10.84-7.22-24.71-13.18-33.64-22.42h1.02c3.28-4.03,7.79-5.18,12.23-8.16,15.77-10.56,32-21.47,47.92-32.62,41.76-29.27,84.71-58.37,126.43-87.68,14.53-10.21,29.37-19.91,43.84-29.56,6.39-4.26,16.66-6.39,17.33-16.31-3.52-5.62-2.12-14.18-4.08-21.41-5.52-20.46-6.65-42.63-12.23-64.23-17.16-66.36-25.68-137.23-39.76-204.91h4.07c5.76,3.61,13.93,2.06,21.41,4.08,22.06,5.96,46.04,7.22,69.33,13.25,34.6,8.95,71.41,11.22,107.06,20.39,16.75,4.31,35.19,5.42,53.01,10.19,12.4,3.32,27.04,7.02,40.79,8.16Q1655.775,978.875,1739.89,855l1.02,1.019c5.6,4.565,8.04,12.326,12.24,18.351,10.27,14.731,20.41,29.563,30.58,44.856,27.29,41,58.1,80.914,84.63,122.334,9.68,15.12,19.83,29.25,29.57,43.84,4.61,6.92,6.06,16.95,17.33,17.33,6.98-4.38,17.57-2.78,26.51-5.1,20.49-5.31,42.6-6.64,64.23-12.23,64.42-16.65,133.26-24.76,198.81-38.74v5.1c-2.7,4.31-1.52,10.62-3.06,16.31-4.07,15.11-4.84,31.82-9.17,47.91-13.94,51.75-18.35,108.26-31.61,160.06-4.27,16.68-5.09,33.27-9.18,48.93-1.14,4.4-3.18,15.44-2.03,17.33,3.86,4.64,10.15,6.78,15.29,10.2,12.42,8.26,25.19,16.7,37.72,25.49,42.74,29.94,86.15,60.64,129.49,89.71,15.87,10.65,31.41,21.04,46.9,31.6,5.48,3.74,15.41,7.18,18.35,13.26h-1.02c-5.75,5.64-13.66,8.53-20.39,13.25-14.69,10.29-29.59,20.42-44.86,30.58-40.01,26.63-79.91,55.06-119.29,82.58-15.97,11.16-32.28,21.64-47.92,32.62-5.64,3.96-13.26,4.57-14.27,13.26Q2177.795,1830.605,2205.83,1976.41ZM1739.89,939.616c-4.19,12.818-22.57,33.443-30.58,44.856-8.22,11.7-14.45,23.758-22.43,35.678-14.74,22.01-31.98,43.3-45.88,66.27h4.07c9.56-6.7,26.85-4.2,39.77-7.14,48.9-11.1,112.48-1.71,154.97,7.14v-1.02c-8.89-7.19-13.93-20.94-20.39-30.58-16.23-24.24-34.26-48.46-50.98-72.387C1758.89,968.758,1751.88,950.9,1739.89,939.616ZM1336.15,1106.81c1.21,12.8,5.23,26.29,8.16,37.72q1.02,9.675,2.03,19.37c6.27,23.18,9.3,50,15.3,73.4,2.84,11.09,3.52,21.66,6.12,31.6,1.21,4.64-.46,9.94,3.05,12.24,5.24-16.18,21.06-30,30.59-42.82,19.84-26.7,43.58-49.53,70.35-69.32,12.78-9.46,24.93-21.12,39.76-28.55v-1.02h-5.09c-4.72-2.85-11.28-1.53-17.34-3.06-12.11-3.04-24.63-3.77-37.72-7.13-25.49-6.56-52.81-8.45-79.53-15.3C1360.84,1111.13,1348.65,1107.52,1336.15,1106.81Zm806.47,0c-8.17,5.14-27.12,4.29-37.72,7.13-30.27,8.13-64.77,11.4-94.82,19.37-11,2.92-30.61,1.68-38.75,7.14,43.66,25.07,78.99,61.09,111.13,97.87,8.05,9.21,15.67,20.47,22.44,30.58,1.24,1.87,6.76,11.91,7.13,10.2,2.93-4.63,1.4-11.14,3.06-17.33,3.5-13.02,4.46-27.36,8.16-41.8,5.25-20.48,6.47-42.7,12.23-64.23,3.96-14.8,8.53-31.98,9.18-48.93h-2.04Zm-425.16,16.31c-6.38,3.77-30.42,1.89-39.76,4.08-8.56,2-18.11,2.35-27.53,5.09q-20.385,6.63-40.78,13.26l1.02,1.02c7.42,7.36,18.69,11.45,27.53,17.33,20.87,13.89,41.66,28.11,63.21,41.8,9.95,6.31,30.4,24.19,41.8,26.5,7.04-8.48,27.69-17.75,37.73-24.46,30.24-20.25,60.43-42.16,91.76-61.17-1.04-.82-0.28-0.36-2.04-1.02-7.4-5.7-18.66-5.95-28.55-9.18C1806.77,1124.91,1764.62,1123.36,1717.46,1123.12Zm-144.77,37.72c-4.39,4.09-10.97,4.91-16.32,8.16-14.19,8.6-29.18,16.93-42.82,26.5-38.59,27.09-70.66,63.29-96.86,102.97-9.08,13.76-21.87,29.58-26.5,46.89h8.15c8.76-5.79,26.93-5.18,38.75-8.15,21.19-5.33,45.94-10.01,67.29-15.29,11.36-2.82,30.76-2.37,38.74-8.16h1.02c0.79-15.98,6.15-33.51,10.19-47.92,2.95-10.52,2.18-19.18,5.1-29.56,6.76-24.05,10.74-50.81,16.32-75.44h-3.06Zm334.41,0c0.56,11.44,4.34,23.71,7.14,33.64,5.05,17.95,6.2,37.29,11.21,54.03,6.61,22.07,5.62,45.45,13.26,66.27,10.61,1.02,21.44,4.54,30.58,7.14,15.04,4.26,30.62,5.33,44.86,9.17,26.38,7.11,54.04,7.34,78.51,16.31v-3.06c-5.52-5.92-6.87-15.28-11.21-22.42-7.79-12.8-15.77-26.34-24.47-38.74-26.46-37.68-62.22-69.22-100.94-94.81C1941.7,1178.89,1925.26,1165.67,1907.1,1160.84Zm-292.61,45.88q-7.14,34.14-14.28,68.3h2.04c6.21-7.4,40.86-21.82,52-25.49,7.22-2.37,13.28-1.05,18.35-5.09C1651.51,1234.66,1635.04,1217.3,1614.49,1206.72Zm252.85,0c-3.53,4.35-8.41,4.97-13.26,8.15-9.8,6.43-19.6,15.04-29.56,21.41-4.44,2.84-10.07,4.15-13.26,8.16l-1.02,1.02c20.03,2.06,40.34,12.19,56.08,20.38,5.06,2.64,9.28,6.42,15.29,8.16v-2.04c-2.06-3.37-.79-7.62-2.04-12.23-2.65-9.84-5.55-22.39-8.15-32.62C1869.63,1220.08,1870.95,1211.94,1867.34,1206.72Zm-139.68,77.48c-7.09,4.14-18.75.97-27.53,3.05-11.54,2.74-23.97,4.68-34.66,9.18-68.16,28.69-112.58,72-140.7,140.69-5.44,13.26-6.77,27.82-10.2,42.81-8.51,37.23,1.14,84.42,12.24,110.11,6.42,14.86,10.25,28.71,18.35,41.79,26.68,43.14,62.66,71.97,112.15,92.77,21.48,9.03,43.65,16.28,75.45,16.32l21.41,1.02c20.06-4.46,39.41-4.03,57.09-10.2,67.4-23.5,115.65-70.55,142.74-134.57,21.89-51.74,19.32-120.84-2.04-170.25C1915.23,1341.93,1846.24,1283.99,1727.66,1284.2Zm-226.34,86.65c-7.52,4.94-37.51,8.3-47.92,11.21-6.02,1.69-14.45.42-17.33,5.1,6.08,4.97,8.94,13.56,13.25,20.39,7,11.09,19.62,22.41,23.45,35.68,0.82-1.04.36-.28,1.02-2.04,5.89-8.09,6.27-22.12,10.19-31.6,5.62-13.57,13.88-26.17,20.4-38.74h-3.06Zm478.17,1.02c1.62,6.75,6.41,11.01,9.18,16.31,4.77,9.14,8.47,19.45,12.23,28.55,3.32,8.02,3.17,19.75,8.16,26.5l1.02-1.02c9.48-20.83,26.19-37.09,36.7-57.09h-5.1c-4.17-2.82-9.43-1.5-15.29-3.06C2011.91,1378.22,1994.92,1373.97,1979.49,1371.87Zm-604.6,9.17c-20.85,51.36-31.69,141.77-15.29,207.98,3.58,14.47,5.39,26.75,10.19,38.74,2.19,5.43,1.35,12.51,6.12,15.29v-1.02c2.67-2.72,2.88-5.79,5.1-9.18,9.03-13.75,18.19-27.57,27.53-41.8,12.33-18.79,24.97-37.92,37.72-57.09,4.59-6.89,12.32-14.28,15.29-22.42-7.51-6.1-10.85-17.28-16.31-25.49-15.08-22.66-31.61-45.83-46.9-68.3C1390.25,1405.85,1385.07,1390.83,1374.89,1381.04Zm732.04,0c-18.14,31.7-41.02,61.55-61.17,91.76-6.27,9.4-17.38,33.06-25.49,38.74,6.97,14.44,18.19,26.6,26.51,39.76,19.84,31.36,41,61.36,60.15,92.77h1.02v-2.04c6.37-8.26,6.73-21.73,10.2-32.63,14.98-47,17.04-125.27,4.08-176.36C2118.35,1417.74,2115.74,1390.71,2106.93,1381.04Zm-792.19,30.59c-12.75,12.96-31.58,21.34-46.9,31.6-33.01,22.09-65.32,46.68-98.9,68.31v1.02c12.61,3.91,30.7,21.1,41.8,28.54,22.58,15.14,44.68,28.75,67.29,43.84,12.07,8.05,23.41,18.51,36.71,25.48v-4.07c-5.71-8.59-3.53-23.33-6.12-34.67-5.44-23.8-7.88-83.84-2.04-110.1q3.06-18.855,6.12-37.72C1313.65,1420.24,1316.27,1414.74,1314.74,1411.63Zm852.35,2.04c9.76,47.44,17.35,116.39,5.1,168.21-2.41,10.17-1.94,18.96-5.1,27.52h2.04c10.96-10.79,26.78-17.86,39.76-26.5,20.89-13.9,41.38-29.96,62.19-43.84,8.75-5.83,37.37-19.99,40.79-28.54-13.13-4.12-24.82-17.19-35.69-24.47-24.95-16.73-49.84-31.81-74.43-48.93C2191.26,1429.81,2179.72,1417.95,2167.09,1413.67Zm-694.32,168.21c-11.48,20.1-27.17,37.47-37.72,58.11,7.14,0.17,13.65,2.56,19.37,4.08,16.35,4.33,32.07,5.67,47.92,9.17v-2.04c-8.9-9.69-17.42-32.56-22.43-45.87C1477.14,1597.98,1477.34,1587.6,1472.77,1581.88Zm535.27,1.02c-1.12,22.3-19.92,53.1-28.55,70.34h4.08c5.07-3.46,12.48-2.18,19.37-4.08,13.51-3.72,28.74-8.35,43.84-9.17v-1.02c-10.36-10.53-16.15-25.1-24.47-37.72l-6.12-6.12C2013.24,1590.72,2012.99,1585.29,2008.04,1582.9Zm-464.92,127.43c-18.45-1.81-37.64-7.52-55.06-12.23-20.6-5.58-40.62-5.46-61.17-11.22-11.81-3.3-24.11-7.6-37.72-8.15v1.02c6.75,7.24,8.99,18.86,14.27,27.52,7.65,12.56,15.83,25.66,24.47,37.72,35.6,49.68,88.85,91.87,146.82,119.28v-3.06c-6.13-9.67-6.65-35.77-10.2-48.93-4.21-15.63-5.23-33.35-10.2-49.96C1549.74,1746.96,1544.72,1727.25,1543.12,1710.33Zm395.59,0q-15.81,76.965-31.61,153.94h2.04c4.9-4.57,12.4-5.57,18.35-9.17,13.12-7.95,27.13-15.6,39.76-24.47,38.46-27,70.68-62.5,96.86-101.95,9.73-14.65,23.59-31.43,28.55-49.95h-2.04Q2014.675,1694.525,1938.71,1710.33Zm-569.94,31.61q-16.815,87.66-33.64,175.34h1.02q86.655-16.815,173.32-33.64v-1.02c-12.97-4.21-23.34-16.03-33.64-23.45-27.78-19.97-57.36-46.98-77.49-74.42-6.46-8.81-13.43-17.62-19.37-26.5C1375.38,1752.88,1373.26,1746.37,1368.77,1741.94Zm743.26,1.01c-5.12,16.12-20.2,29.16-29.57,41.8-21.93,29.62-48.49,54.9-78.5,76.46-9.94,7.14-18.93,15.84-30.59,21.41,1.04,0.82.28,0.36,2.04,1.02,10.75,6.82,34.96,5.43,48.94,9.18,22.08,5.92,45.98,7.26,69.33,13.25,15.96,4.09,34.42,9.2,52,10.19v-1.02c-5.49-8.98-3.24-23.7-6.12-34.66-8.4-31.97-12.18-68.02-20.39-99.9C2116.37,1769.82,2117.46,1750.82,2112.03,1742.95Zm-511.82,8.16c0,14.78,5.6,31.54,9.18,44.86,1.94,7.2.99,17.23,5.1,22.43,5.31-6.49,15.31-9.53,22.43-14.28,10.06-6.7,21.44-18.84,33.64-22.43v-1.01C1644.5,1775.44,1621.29,1762.06,1600.21,1751.11Zm280.38,0c-15.82,15.43-46.85,20.36-69.33,29.57,4.64,4.61,10.67,6.44,16.32,10.19,13.26,8.82,26.22,20.08,40.78,27.53q4.59-24.465,9.17-48.94C1878.77,1765,1882.66,1755.33,1880.59,1751.11Zm-140.7,41.8q-65.235,42.81-130.5,85.63v1.02c8.43,1.15,16.28,5.6,23.45,8.16,9.3,3.31,19.51,4.76,28.55,7.14,41.43,10.89,119.06,9.57,159.05-1.02,15.04-3.99,42.4-6.59,52-15.3-31.23-17.89-60.75-39.17-90.74-59.13C1772.25,1813.13,1750.75,1795.12,1739.89,1792.91ZM1641,1938.69c8.81,14.36,20.27,27.95,29.56,41.8,13.76,20.5,25.67,40.94,39.77,61.17,7.94,11.4,24.98,29.98,29.56,42.82,0.82-1.04.37-.28,1.02-2.04,5.48-4.01,7.31-10.67,11.22-16.31,7.2-10.39,14.16-20.72,21.41-31.61,14.92-22.42,31.12-44.16,45.88-67.28,5.79-9.08,15.75-17.79,19.37-28.55h-2.04c-8.94,6.01-28.3,4.35-40.78,7.14-22.43,5.01-65.49,7.29-89.72,2.04l-20.39-1.02Zm229.4,739.12H1609.39v-2.04c4.64-6.29,5.87-14.88,9.18-22.43,8.59-19.62,16.61-39.76,25.49-60.15,21.19-48.7,43.08-99.11,64.23-147.82,7.82-18.02,14.9-35.77,22.43-53.02,2.8-6.42,3.34-18.1,11.21-19.37,1.85,8.37,7.02,16.24,10.2,23.45,8.18,18.55,15.59,37.19,23.45,55.05,22.3,50.72,41.08,101.14,63.21,151.9,6.78,15.55,13.33,31.68,20.39,47.92C1862.71,2659.42,1868.85,2668.13,1870.4,2677.81ZM808.018,2375.02h3.059c1.967,5.7,7,7.99,10.2,12.24,11.545,15.32,25.375,28.74,36.7,43.83,20.127,26.81,43.112,50.66,63.212,77.48,4.606,6.15,11.713,11.18,16.313,17.33,8.341,11.16,18.342,20.76,26.509,31.61,3.336,4.43,5.55,9.14,10.2,12.23V2384.2h46.9V2688h-1.02c-9.52-14.6-24.283-25.99-34.669-39.76-27.549-36.53-60.155-68.45-87.682-105-12.09-16.06-32.426-31.04-41.8-48.94h-1.02v183.51h-46.9V2375.02Zm1804.612,6.12c15.76-.19,30.91.21,43.85,4.08,52.85,15.83,87.55,49.48,106.03,99.91,6.7,18.28,12.29,50,6.12,74.42-2.59,10.23-5.09,21.14-9.18,30.58-16.59,38.26-48.58,66.71-87.68,82.58-11.09,4.5-24.23,5.44-36.71,8.15-22.13,4.82-55.16-4.68-68.31-10.19-45.68-19.14-73.89-47.83-89.72-96.85-5.05-15.66-10.22-44.85-5.09-66.27q3.555-13.755,7.13-27.52c16.04-39.24,45.07-71.03,83.61-87.68,9.67-4.18,21.02-6.52,31.6-9.17C2599.69,2381.83,2608.3,2383.81,2612.63,2381.14ZM390,2383.18H585.755v45.88H510.308v201.85h75.447v45.88H390v-45.88h73.408V2429.06H390v-45.88Zm856.43,1.02h195.75v45.88h-75.44v247.73h-46.9V2430.08h-73.41V2384.2Zm882.93,191.66v101.95h-47.91V2385.22c7.16-2.37,19.61-1.02,28.54-1.02h60.16c4.59,2.73,27.83,3.02,34.66,5.1,16.46,5.01,29.61,13.2,41.8,22.42,21.28,16.1,34.17,62.88,22.43,98.89-8.03,24.65-21.6,43.59-43.84,54.03q35.685,56.58,71.37,113.17h-55.05c-3.05-9.96-12.11-18.26-17.34-26.51-10.55-16.67-20.9-32.88-31.6-48.94-3.25-4.86-12.35-24.44-16.32-26.5h-46.9Zm827.89-191.66H3153v45.88h-75.45v247.73h-46.9V2430.08h-73.4V2384.2Zm-350.73,42.82c-6.75,4.22-20.93,4.8-28.55,8.15-25.96,11.44-45.93,36.91-55.06,65.25-19.2,59.62,19.66,109.79,58.12,126.41,13.75,5.95,36.92,13.31,58.11,8.16,59.63-14.5,105.18-78.12,73.41-149.86-11.81-26.67-37.5-47.26-67.29-56.07C2634.41,2425.85,2619.95,2426.94,2606.52,2427.02Zm-477.16,3.06V2531c52.89,0.75,84.35,2.14,94.82-40.78,2.64-10.8-1.48-23.32-4.07-29.56C2205.66,2425.98,2177.42,2429.41,2129.36,2430.08Zm-331.35,200.83c-0.95-8.2-6.01-16.14-9.18-22.43-5.42-10.76-8.75-23.15-13.25-33.64-12.94-30.17-23.09-66.53-37.73-97.87h-1.01c-1.7,13.64-11.26,27.95-16.32,39.76-9.52,22.22-15.79,43.91-25.49,66.26-6.64,15.31-15.91,31.14-20.39,47.92h123.37Z" transform="translate(-390 -855)"/>
    </svg>

        )
}
