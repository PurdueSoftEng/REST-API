--
--CREATE TABLE `app`.`packages` (
--  `id` BIGINT NOT NULL AUTO_INCREMENT,
--  `view_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
--  `url` VARCHAR(2083) NOT NULL,
--  `user_agent` VARCHAR(2083) NOT NULL,
--  `device_type` TINYINT NOT NULL DEFAULT '0',
--  PRIMARY KEY (`id`)
--);



--
-- Name: postgres; Type: DATABASE; Schema: -; Owner: postgres
--

CREATE DATABASE postgres WITH TEMPLATE = template0 ENCODING = 'UTF8' LC_COLLATE = 'English_United States.1252' LC_CTYPE = 'English_United States.1252';


ALTER DATABASE postgres OWNER TO postgres;

\connect postgres


--
-- Name: user; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user (
    users_id SERIAL PRIMARY KEY,
    username VARCHAR(50)
);


ALTER TABLE public.user OWNER TO postgres;

--
-- Name: request; Type: TABLE; Schema: public; Owner: postgres
--


CREATE TABLE public.request (
    request_id SERIAL PRIMARY KEY,
    request_type VARCHAR(50)
);


ALTER TABLE public.request OWNER TO postgres;


--
-- Name: group; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.group (
    group_id SERIAL PRIMARY KEY,
    group_name VARCHAR(50)
);


ALTER TABLE public.group OWNER TO postgres;


--
-- Name: package; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.package (
    package_id SERIAL PRIMARY KEY,
    package_name VARCHAR(50),
    unparsed_m_info text [],
    metric_one integer NOT NULL,
    metric_two integer NOT NULL,
    metric_three integer NOT NULL,
    metric_four integer NOT NULL,
    metric_five integer NOT NULL,
    metric_six integer NOT NULL,
    metric_seven integer NOT NULL,
    total_score integer NOT NULL

);


ALTER TABLE public.package OWNER TO postgres;

--
-- Name: history_log; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.history_log (
    history_id SERIAL PRIMARY KEY,
    request_id integer NOT NULL,
    package_id integer NOT NULL,
    users_id integer NOT NULL
);


ALTER TABLE public.history_log OWNER TO postgres;

--
-- Name: registry; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.registry (
    registry_id SERIAL PRIMARY KEY,
    group_id integer NOT NULL,
    package_id integer NOT NULL,
    use_count integer NOT NULL
);


ALTER TABLE public.registry OWNER TO postgres;

--
-- Data for Name: user; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.user (users_id, username) FROM stdin;
\.
COPY public.user (users_id, username) FROM '$$PATH$$/3057.dat';

--
-- Data for Name: request; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.request (request_id, request_type) FROM stdin;
\.
COPY public.request (request_id, request_type) FROM '$$PATH$$/3065.dat';

--
-- Data for Name: group; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.group (group_id, group_name) FROM stdin;
\.
COPY public.group (group_id, group_name) FROM '$$PATH$$/3059.dat';

--
-- Data for Name: package; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.package (package_id, package_name, unparsed_m_info, metric_one, metric_two, metric_three, metric_four, metric_five, metric_six, metric_seven, total_score) FROM stdin;
\.
COPY public.package (package_id, package_name, unparsed_m_info, metric_one, metric_two, metric_three, metric_four, metric_five, metric_six, metric_seven, total_score) FROM '$$PATH$$/3067.dat';

--
-- Data for Name: history_log; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.history_log (history_id, request_id, package_id, users_id) FROM stdin;
\.
COPY public.history_log (history_id, request_id, package_id, users_id) FROM '$$PATH$$/3069.dat';

--
-- Data for Name: registry; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.registry (registry_id, group_id, package_id, use_count) FROM stdin;
\.
COPY public.registry (registry_id, group_id, package_id, use_count) FROM '$$PATH$$/3055.dat';


--
-- Name: history_log history_log_request_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.history_log
    ADD CONSTRAINT history_log_request_id FOREIGN KEY (request_id) REFERENCES public.request(request_id) ON UPDATE CASCADE ON DELETE RESTRICT;

--
-- Name: history_log history_log_package_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.history_log
    ADD CONSTRAINT history_log_package_id FOREIGN KEY (package_id) REFERENCES public.package(package_id) ON UPDATE CASCADE ON DELETE RESTRICT;

--
-- Name: registry registry_history_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.registry
    ADD CONSTRAINT registry_history_id FOREIGN KEY (history_id) REFERENCES public.history_log(history_id) ON UPDATE CASCADE ON DELETE RESTRICT;

--
-- Name: registry registry_group_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.registry
    ADD CONSTRAINT registry_group_id FOREIGN KEY (group_id) REFERENCES public.group(group_id) ON UPDATE CASCADE ON DELETE RESTRICT;

--
-- Name: registry registry_package_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.registry
    ADD CONSTRAINT registry_package_id FOREIGN KEY (package_id) REFERENCES public.package(package_id) ON UPDATE CASCADE ON DELETE RESTRICT;

--
-- PostgreSQL database dump complete
--


